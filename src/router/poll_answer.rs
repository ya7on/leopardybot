use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::texts::TextFormatter;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct PollAnswerHandler;

#[async_trait::async_trait]
impl RouteHandler for PollAnswerHandler {
    async fn handle(
        &self,
        db: &DatabaseConnection,
        client: &Client,
        update: &Update,
    ) -> Result<()> {
        let poll_answer = update.poll_answer.as_ref().ok_or_else(|| {
            Error(format!(
                "Invalid request. Missing poll_answer. {:?}",
                update
            ))
        })?;

        let player = GameHandler::get_or_create_player(db, poll_answer.user.id).await?;
        let poll = GameHandler::get_poll(db, poll_answer.poll_id.clone()).await?;
        let game = GameHandler::get_by_id(db, poll.game_id as usize).await?;

        if poll_answer
            .option_ids
            .contains(&(poll.correct_option_id as usize))
        {
            GameHandler::add_user_poll_answer(
                db,
                player.telegram_id as isize,
                poll.id.clone(),
                true,
            )
            .await?;
            GameHandler::increase_player_score(db, player.telegram_id as isize, 1).await?;
        } else {
            GameHandler::add_user_poll_answer(
                db,
                player.telegram_id as isize,
                poll.id.clone(),
                false,
            )
            .await?;
        };

        if game.model.game_mode == Gamemodes::Singleplayer {
            let question = if let Some(question) =
                GameHandler::get_new_question(db, poll_answer.user.id).await?
            {
                GameHandler::mark_quiz_as_played(db, poll_answer.user.id, question.id as isize)
                    .await?;
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
                    game.model.chat_id as isize,
                    &question.text,
                    &question.options.iter().map(|i| i.text.clone()).collect(),
                    question.explanation,
                    question.correct_answer_id,
                    None,
                )
                .await?;
            GameHandler::mark_poll_as_handled(db, poll.id.clone()).await?;
            let poll = response
                .poll
                .as_ref()
                .ok_or_else(|| Error(format!("Empty poll field. {:?}", response)))?;
            game.register_poll(db, &poll, response.message_id).await?;
        }

        Ok(())
    }
}
