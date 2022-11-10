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
        _: &Client,
        update: &Update,
    ) -> crate::error::Result<()> {
        if let Some(poll_answer) = &update.poll_answer {
            let player = GameHandler::get_or_create_player(db, poll_answer.user.id).await?;
            let poll = GameHandler::get_poll(db, poll_answer.poll_id.clone()).await?;

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
        }

        Ok(())
    }
}
