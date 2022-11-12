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
        if let Some(message) = &update.message {
            GameHandler::register_chat(db, message.chat.id).await?;

            if !GameHandler::exists(db, message.chat.id).await? {
                let game =
                    GameHandler::create(db, message.chat.id, Gamemodes::Singleplayer).await?;
                let question = GameHandler::get_question(db).await?;
                let response = client
                    .send_quiz(
                        message.chat.id,
                        &question.text,
                        &question.options.iter().map(|i| i.text.clone()).collect(),
                        question.correct_answer_id,
                        None,
                    )
                    .await?;
                let result = response.result.ok_or_else(|| {
                    // FIXME error handle
                    Error::SerializationError("Empty result field".to_owned())
                })?;
                let poll = result
                    .poll
                    .ok_or_else(|| Error::SerializationError("Empty poll field".to_owned()))?;
                game.register_poll(db, &poll).await?;
            } else {
                client
                    .send_message(
                        message.chat.id,
                        &TextFormatter::single_game_already_started()?,
                    )
                    .await?;
            }
        }
        Ok(())
    }
}
