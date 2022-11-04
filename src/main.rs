#[macro_use]
extern crate log;

use actix_rt::time;
use actix_web::{middleware, web, App, HttpServer};
use leopardybot::api::handler;
use leopardybot::conf::get_config;
use leopardybot::error::{Error, Result};
use leopardybot::game::base::GameHandler;
use leopardybot::telebot::client::Client;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let c = get_config();

    actix_rt::spawn(async {
        let opt = ConnectOptions::new(c.db.clone());
        let db = Database::connect(opt).await.unwrap(); // TODO clone
        let client = Client::new(
            &c.telegram_token,
            &format!("https://{}/api/v1/telegram", &c.host),
        )
        .await
        .unwrap(); // TODO clone

        let mut interval = time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            let result: Result<()> = async {
                let polls = GameHandler::get_unhandled_polls(&db).await?;
                for poll in polls.iter() {
                    let game = GameHandler::get_by_id(&db, poll.game_id as usize).await?;
                    let chat_id = game.model.chat_id;
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
                        Error::SerializationError(format!("Empty result field"))
                    })?;
                    GameHandler::mark_poll_as_handled(&db, poll.id.clone()).await?;
                    let poll = result
                        .poll
                        .ok_or_else(|| Error::SerializationError(format!("Empty poll field")))?;
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

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/v1/telegram").to(handler))
            .data_factory(|| async {
                Ok::<Client, Error>(
                    Client::new(
                        &c.telegram_token,
                        &format!("https://{}/api/v1/telegram", &c.host),
                    )
                    .await?,
                )
            })
            .data_factory(|| async {
                info!("Connection to Database");
                let opt = ConnectOptions::new(c.db.clone());
                let db = Database::connect(opt).await.unwrap();

                info!("Running migrations");
                Migrator::up(&db, None).await.unwrap();

                Ok::<DatabaseConnection, Error>(db)
            })
    })
    .bind(("0.0.0.0", c.port))?
    .workers(c.workers)
    .run()
    .await
}
