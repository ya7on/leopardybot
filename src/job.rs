use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::telebot::client::Client;
use actix_rt::time;
use sea_orm::DatabaseConnection;
use std::time::Duration;

pub async fn run(db: DatabaseConnection, client: Client) {
    actix_rt::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            let result: Result<()> = async {
                let polls = GameHandler::get_unhandled_polls(&db).await?;
                for poll in polls.iter() {
                    let game = GameHandler::get_by_id(&db, poll.game_id as usize).await?;
                    let chat_id = game.model.chat_id;
                    if game.get_rounds(&db).await? > 5 {
                        // TODO вынести в настройку
                        client
                            .send_message(chat_id as isize, &"Игра завершена".to_string())
                            .await?;
                        game.end_game(&db).await?;
                        GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                        return Ok(());
                    }
                    client
                        .send_message(chat_id as isize, &"Раунд завершен".to_string())
                        .await?;
                    let question = GameHandler::get_question(&db).await?;
                    let response = client
                        .send_quiz(
                            chat_id as isize,
                            &question.text,
                            &question.options.iter().map(|i| i.text.clone()).collect(),
                            question.correct_answer_id,
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
                    game.register_poll(&db, &poll).await?;
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
