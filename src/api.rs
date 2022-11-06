use crate::conf::get_config;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::telebot::typings::output::{Message, PollAnswer};
use crate::texts::TextFormatter;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use regex::Regex;
use sea_orm::DatabaseConnection;

fn parse_command(text: &str) -> Result<Option<String>> {
    let re = Regex::new(r"(/[a-zA-Z0-9_]+)(@.+)?")
        .map_err(|err| Error::SerializationError(format!("Invalid regex. {}", err)))?;
    Ok(re.captures(text).map(|c| c[1].to_string()))
}

async fn handle_message(
    client: &web::Data<Client>,
    db: &web::Data<DatabaseConnection>,
    message: &Message,
) -> Result<()> {
    let c = get_config();

    if GameHandler::register_chat(db, message.chat.id).await? {
        client
            .send_message(message.chat.id, &TextFormatter::new_chat()?)
            .await?;
    }

    if let Some(text) = &message.text {
        if let Some(command) = parse_command(text)? {
            match command.as_str() {
                "/start" => {
                    client
                        .send_message(message.chat.id, &TextFormatter::start()?)
                        .await?;
                }
                "/help" => {
                    client
                        .send_message(message.chat.id, &TextFormatter::help()?)
                        .await?;
                }
                "/play" => {
                    // FIXME
                    if !GameHandler::exists(db, message.chat.id).await? {
                        let game = GameHandler::create(db, message.chat.id).await?;
                        let mut question = GameHandler::get_question(db).await?;
                        question.text = format!("[1/{}] {}", c.quiz_rounds_count, question.text);
                        let response = client
                            .send_quiz(
                                message.chat.id,
                                &question.text,
                                &question.options.iter().map(|i| i.text.clone()).collect(),
                                question.correct_answer_id,
                            )
                            .await?;
                        let result = response.result.ok_or_else(|| {
                            // FIXME error handle
                            Error::SerializationError("Empty result field".to_owned())
                        })?;
                        let poll = result.poll.ok_or_else(|| {
                            Error::SerializationError("Empty poll field".to_owned())
                        })?;
                        game.register_poll(db, &poll).await?;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

async fn handle_poll_answer(
    _: &web::Data<Client>,
    db: &web::Data<DatabaseConnection>,
    poll_answer: &PollAnswer,
) -> Result<()> {
    let player = GameHandler::get_or_create_player(db, poll_answer.user.id).await?;
    let poll = GameHandler::get_poll(db, poll_answer.poll_id.clone()).await?;

    if poll_answer
        .option_ids
        .contains(&(poll.correct_option_id as usize))
    {
        GameHandler::add_user_poll_answer(db, player.telegram_id as isize, poll.id.clone(), true)
            .await?;
        GameHandler::increase_player_score(db, player.telegram_id as isize, 1).await?;
    } else {
        GameHandler::add_user_poll_answer(db, player.telegram_id as isize, poll.id.clone(), false)
            .await?;
    };

    Ok(())
}

pub async fn handler(
    update: Json<Update>,
    db: web::Data<DatabaseConnection>,
    client: web::Data<Client>,
) -> HttpResponse {
    match update.into_inner() {
        Update {
            message: Some(message),
            ..
        } => {
            if let Err(err) = handle_message(&client, &db, &message).await {
                error!("{:?}", err);
            };
        }
        Update {
            poll: Some(poll), ..
        } => {
            info!("POLL {:?}", poll);
        }
        Update {
            poll_answer: Some(poll_answer),
            ..
        } => {
            if let Err(err) = handle_poll_answer(&client, &db, &poll_answer).await {
                error!("{:?}", err);
            }
        }
        _ => {
            info!("UNKNOWN");
        }
    };
    HttpResponse::Ok().finish()
}
