use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::texts::TextFormatter;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct PlaySingleCommand;

#[async_trait::async_trait]
impl RouteHandler for PlaySingleCommand {
    async fn handle(
        &self,
        db: &DatabaseConnection,
        client: &Client,
        update: &Update,
    ) -> Result<()> {
        let message = update
            .message
            .as_ref()
            .ok_or_else(|| Error(format!("Invalid request. Missing message. {:?}", update)))?;
        let user = message
            .from
            .as_ref()
            .ok_or_else(|| Error(format!("Invalid request. Missing from. {:?}", update)))?;

        GameHandler::register_chat(db, message.chat.id).await?;
        GameHandler::get_or_create_player(db, user.id).await?;

        if !GameHandler::exists(db, message.chat.id).await? {
            let game = GameHandler::create(db, message.chat.id, Gamemodes::Singleplayer).await?;

            let question =
                if let Some(question) = GameHandler::get_new_question(db, user.id).await? {
                    GameHandler::mark_quiz_as_played(db, user.id, question.id as isize).await?;
                    question
                } else {
                    client
                        .send_message(
                            game.model.chat_id as isize,
                            &TextFormatter::cannot_find_new_quiz()?,
                        )
                        .await?;
                    GameHandler::get_question(db).await?
                };
            let response = client
                .send_quiz(
                    message.chat.id,
                    &question.text,
                    &question.options.iter().map(|i| i.text.clone()).collect(),
                    question.explanation,
                    question.correct_answer_id,
                    None,
                )
                .await?;
            let poll = response
                .poll
                .as_ref()
                .ok_or_else(|| Error(format!("Empty poll field. {:?}", response)))?;
            game.register_poll(db, &poll, response.message_id).await?;
        } else {
            client
                .send_message(
                    message.chat.id,
                    &TextFormatter::single_game_already_started()?,
                )
                .await?;
        }
        Ok(())
    }
}
