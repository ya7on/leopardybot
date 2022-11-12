use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
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
        if let Some(poll_answer) = &update.poll_answer {
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
                let question = GameHandler::get_question(&db).await?;
                let response = client
                    .send_quiz(
                        game.model.chat_id as isize,
                        &question.text,
                        &question.options.iter().map(|i| i.text.clone()).collect(),
                        question.correct_answer_id,
                        None,
                    )
                    .await?;
                let result = response.result.ok_or_else(|| {
                    // FIXME error handle
                    Error::SerializationError("Empty result field".to_string())
                })?;
                GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                let poll = result
                    .poll
                    .ok_or_else(|| Error::SerializationError("Empty poll field".to_string()))?;
                game.register_poll(&db, &poll, result.message_id).await?;
            }
        }

        Ok(())
    }
}
