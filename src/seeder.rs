use crate::entities::quiz;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvQuizRow {
    question: String,
    correct_answer: String,
    answer_2: String,
    answer_3: String,
    answer_4: String,
}

pub async fn run(db: DatabaseConnection) -> Result<()> {
    info!("Seeding questions");
    let file = std::fs::File::open("questions/questions.csv")
        .map_err(|err| Error::SerializationError(format!("Cannot open file. {}", err)))?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(file);
    let mut questions = Vec::new();
    for record in reader.deserialize::<CsvQuizRow>() {
        let record = record
            .map_err(|err| Error::SerializationError(format!("Cannot parse csv row. {}", err)))?;
        questions.push(quiz::ActiveModel {
            text: Set(record.question),
            correct_option: Set(record.correct_answer),
            option2: Set(record.answer_2),
            option3: Set(record.answer_3),
            option4: Set(record.answer_4),
            ..Default::default()
        })
    }
    GameHandler::clear_question(&db).await?;
    for to_insert in questions.chunks(100) {
        GameHandler::insert_questions(&db, to_insert.to_vec()).await?;
    }
    info!("Seeding questions is done");
    Ok(())
}