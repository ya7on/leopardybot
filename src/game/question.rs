use crate::entities::{question, question_option};
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::game::typings::{QuizPoll, QuizPollOption};
use migration::Condition;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

impl GameHandler {
    /// TODO FIXME удалить, когда будет реализован другой метод
    pub async fn get_question(db: &DatabaseConnection) -> Result<QuizPoll> {
        let question = <question::Entity as EntityTrait>::find()
            .one(db)
            .await
            .map_err(|err| {
                error!("Cannot fetch questions from DB. {}", err);
                Error::DatabaseError(format!("Cannot fetch questions from DB. {}", err))
            })?
            .ok_or_else(|| {
                error!("Empty question result");
                Error::DatabaseError("Empty question result".to_string())
            })?;
        let options =
            <question_option::Entity as EntityTrait>::find()
                .filter(Condition::all().add(
                    <question_option::Entity as EntityTrait>::Column::QuestionId.eq(question.id),
                ))
                .all(db)
                .await
                .map_err(|err| {
                    error!("Cannot fetch question options from DB. {}", err);
                    Error::DatabaseError(format!("Cannot fetch question options from DB. {}", err))
                })?;

        Ok(QuizPoll {
            id: question.id,
            text: question.text.clone(),
            options: options
                .iter()
                .map(|option| QuizPollOption {
                    id: option.id,
                    text: option.text.clone(),
                })
                .collect(),
            correct_answer_id: 1, // TODO FIXME
        })
    }
}
