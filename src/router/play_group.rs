use crate::conf::get_config;
use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::texts::TextFormatter;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct PlayGroupCommand;

#[async_trait::async_trait]
impl RouteHandler for PlayGroupCommand {
    async fn handle(
        &self,
        db: &DatabaseConnection,
        client: &Client,
        update: &Update,
    ) -> Result<()> {
        let c = get_config();
        let message = update
            .message
            .as_ref()
            .ok_or_else(|| Error(format!("Invalid request. Missing message. {:?}", update)))?;
        if GameHandler::register_chat(db, message.chat.id).await? {
            client
                .send_message(message.chat.id, &TextFormatter::new_group_chat()?)
                .await?;
        }
        if !GameHandler::exists(db, message.chat.id).await? {
            let game = GameHandler::create(db, message.chat.id, Gamemodes::Multiplayer).await?;

            let mut question = GameHandler::get_question(db).await?;
            question.text = format!("[1/{}] {}", c.quiz_rounds_count, question.text);

            let response = client
                .send_quiz(
                    message.chat.id,
                    &question.text,
                    &question.options.iter().map(|i| i.text.clone()).collect(),
                    question.explanation,
                    question.correct_answer_id,
                    Some(c.quiz_round_time),
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
                    &TextFormatter::group_game_already_started()?,
                )
                .await?;
        }
        Ok(())
    }
}
