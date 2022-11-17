use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct RestartCommand;

#[async_trait::async_trait]
impl RouteHandler for RestartCommand {
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

        if GameHandler::exists(db, message.chat.id).await? {
            let game = GameHandler::get_by_chat_id(db, message.chat.id).await?;
            for poll in game.get_active_polls(db).await? {
                client
                    .delete_message(message.chat.id, poll.message_id as usize)
                    .await?;
                GameHandler::mark_poll_as_handled(db, poll.id.clone()).await?;
            }
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
            let poll = response
                .poll
                .ok_or_else(|| Error("Empty poll field".to_owned()))?;
            game.register_poll(db, &poll, response.message_id).await?;
        } else {
            client
                .send_message(
                    message.chat.id,
                    "Вы не начинали игру. Введите /play чтобы начать",
                )
                .await?;
        }
        Ok(())
    }
}
