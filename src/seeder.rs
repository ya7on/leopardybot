use crate::conf::get_config;
use crate::entities::quiz;
use crate::error::Result;
use crate::game::base::GameHandler;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvQuizRow {
    id: usize,
    question: String,
    correct_answer: String,
    answer_2: String,
    answer_3: String,
    answer_4: String,
    explanation: Option<String>,
}

// FIXME
async fn read_csv(dest: &String) -> Result<Vec<u8>> {
    if dest.starts_with("http") {
        Ok(Vec::from(
            reqwest::get(dest).await?.text().await?.as_bytes(),
        ))
    } else {
        Ok(std::fs::read(dest)?)
    }
}

pub async fn run(db: DatabaseConnection) -> Result<()> {
    info!("Seeding questions");
    let c = get_config();
    let file = read_csv(&c.csv_path).await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .double_quote(true)
        .from_reader(file.as_slice());
    let mut questions = Vec::new();
    for record in reader.deserialize::<CsvQuizRow>() {
        let record = record?;
        if let Some(explanation) = &record.explanation {
            if explanation.len() > 200 {
                panic!("Explanation max len is 200. {:?}", record);
            }
        }
        questions.push(quiz::ActiveModel {
            id: Set(record.id as i32),
            text: Set(record.question),
            correct_option: Set(record.correct_answer),
            option2: Set(record.answer_2),
            option3: Set(record.answer_3),
            option4: Set(record.answer_4),
            explanation: Set(record.explanation),
        })
    }
    for to_insert in questions.chunks(100) {
        GameHandler::insert_questions(&db, to_insert.to_vec()).await?;
    }
    info!(
        "Seeding questions is done. Added {} questions",
        questions.len()
    );
    Ok(())
}
