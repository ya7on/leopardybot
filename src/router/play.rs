use crate::conf::get_config;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct PlayCommand;

#[async_trait::async_trait]
impl RouteHandler for PlayCommand {
    async fn handle(
        &self,
        db: &DatabaseConnection,
        client: &Client,
        update: &Update,
    ) -> Result<()> {
        let c = get_config();
        // FIXME
        if let Some(message) = &update.message {
            GameHandler::register_chat(db, message.chat.id).await?;

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
                let poll = result
                    .poll
                    .ok_or_else(|| Error::SerializationError("Empty poll field".to_owned()))?;
                game.register_poll(db, &poll).await?;
            }
        }
        Ok(())
    }
}
