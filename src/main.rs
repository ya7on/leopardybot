#[macro_use]
extern crate log;

use actix_rt::time;
use actix_web::{middleware, web, App, HttpServer};
use leopardybot::api::handler;
use leopardybot::conf::get_config;
use leopardybot::entities::quiz;
use leopardybot::error::{Error, Result};
use leopardybot::game::base::GameHandler;
use leopardybot::telebot::client::Client;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, Set};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvQuizRow {
    question: String,
    correct_answer: String,
    answer_2: String,
    answer_3: String,
    answer_4: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_module("sqlx::query", log::LevelFilter::Error)
        .init();

    let c = get_config();

    info!("Seeding questions");
    let opt = ConnectOptions::new(c.db.clone());
    let db = Database::connect(opt).await.unwrap(); // TODO clone

    let file = std::fs::File::open("questions/questions.csv").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(file);
    let mut questions = Vec::new();
    for record in reader.deserialize::<CsvQuizRow>() {
        let record = record.unwrap();
        questions.push(quiz::ActiveModel {
            text: Set(record.question),
            correct_option: Set(record.correct_answer),
            option2: Set(record.answer_2),
            option3: Set(record.answer_3),
            option4: Set(record.answer_4),
            ..Default::default()
        })
    }
    GameHandler::clear_question(&db).await.unwrap();
    for to_insert in questions.chunks(100) {
        GameHandler::insert_questions(&db, to_insert.to_vec())
            .await
            .unwrap();
    }
    info!("Seeding questions is done");

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

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/v1/telegram").to(handler))
            .data_factory(|| async {
                Client::new(
                    &c.telegram_token,
                    &format!("https://{}/api/v1/telegram", &c.host),
                )
                .await
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
