use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::telebot::typings::output::Message;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use regex::Regex;
use sea_orm::DatabaseConnection;

const HELP_MESSAGE: &str = r#"HELP MESSAGE"#; // TODO

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
    if GameHandler::register_chat(db, message.chat.id).await? {
        client
            .send_message(message.chat.id, &"СООБЩЕНИЕ ДЛЯ НОВЫХ ЧАТОВ".to_string())
            .await?;
    }

    if let Some(text) = &message.text {
        if let Some(command) = parse_command(text)? {
            println!("{:?}", command);
            match command.as_str() {
                "/help" => {
                    client
                        .send_message(message.chat.id, &HELP_MESSAGE.to_string())
                        .await?;
                }
                "/play" => {
                    // FIXME
                    if !GameHandler::exists(db, message.chat.id).await? {
                        let game = GameHandler::create(db, message.chat.id).await?;
                        let question = GameHandler::get_question(db).await?;
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
            info!("POLL ANSWER {:?}", poll_answer);
        }
        _ => {
            info!("UNKNOWN");
        }
    };
    HttpResponse::Ok().finish()
}
