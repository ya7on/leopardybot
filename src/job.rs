use crate::conf::get_config;
use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::telebot::client::Client;
use crate::texts::TextFormatter;
use actix_rt::time;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tracing::Level;

pub async fn run(db: DatabaseConnection, client: Client) {
    let c = get_config();

    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            let result: Result<()> = async {
                let polls = GameHandler::get_unhandled_polls(&db).await?;
                for poll in polls.iter() {
                    let span = span!(
                        Level::DEBUG,
                        "job",
                        game_id = poll.game_id,
                        poll_id = poll.id,
                    );
                    let _enter = span.enter();

                    let game = GameHandler::get_by_id(&db, poll.game_id as usize).await?;
                    if game.model.game_mode == Gamemodes::Multiplayer {
                        let chat_id = game.model.chat_id;
                        let round_number = game.get_rounds(&db).await?;
                        if round_number >= c.quiz_rounds_count as usize {
                            if let Err(_) = client
                                .send_message(chat_id as isize, &TextFormatter::game_over()?)
                                .await
                            {
                                error!("Cannot send message"); // TODO #22
                                game.end_game(&db).await?;
                                GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                            }
                            game.end_game(&db).await?;
                            GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                            return Ok(());
                        }
                        if let Err(_) = client
                            .send_message(chat_id as isize, &TextFormatter::round_over()?)
                            .await
                        {
                            error!("Cannot send message"); // TODO #22
                            game.end_game(&db).await?;
                            GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                        }
                        let mut question = GameHandler::get_question(&db).await?;
                        question.text = format!(
                            "[{}/{}] {}",
                            round_number + 1,
                            c.quiz_rounds_count,
                            question.text
                        );
                        let response = client
                            .send_quiz(
                                chat_id as isize,
                                &question.text,
                                &question.options.iter().map(|i| i.text.clone()).collect(),
                                question.correct_answer_id,
                                Some(c.quiz_round_time),
                            )
                            .await?;
                        GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                        let poll = response
                            .poll
                            .ok_or_else(|| Error("Empty poll field".to_string()))?;
                        game.register_poll(&db, &poll, response.message_id).await?;
                    }
                }
                Ok(())
            }
            .await;
            if let Err(err) = result {
                error!("{:?}", err);
            }
        }
    });
}
