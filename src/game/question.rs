use crate::entities::quiz;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::game::typings::{QuizPoll, QuizPollOption};
use rand::seq::SliceRandom;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Statement,
};

impl GameHandler {
    /// TODO FIXME удалить, когда будет реализован другой метод
    pub async fn get_question(db: &DatabaseConnection) -> Result<QuizPoll> {
        let sql = "SELECT * FROM public.quiz ORDER BY RANDOM() LIMIT 1;";
        let stmt = Statement::from_sql_and_values(db.get_database_backend(), sql, vec![]);
        let quiz = <quiz::Entity as EntityTrait>::find()
            .from_raw_sql(stmt)
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
        let mut options = vec![
            QuizPollOption {
                is_correct: true,
                text: quiz.correct_option,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz.option2,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz.option3,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz.option4,
            },
        ];
        options.shuffle(&mut rand::thread_rng());

        let correct_answer_id = options
            .iter()
            .position(|i| i.is_correct)
            .ok_or_else(|| Error::SerializationError("Cannot find correct answer".to_string()))?;

        Ok(QuizPoll {
            id: quiz.id,
            text: quiz.text,
            options,
            correct_answer_id,
        })
    }

    // pub async fn clear_question(db: &DatabaseConnection) -> Result<()> {
    //     <quiz::Entity as EntityTrait>::delete_many()
    //         .exec(db)
    //         .await
    //         .map_err(|err| Error::DatabaseError(format!("Cannot delete quiz. {}", err)))?;
    //     Ok(())
    // }

    pub async fn question_exists(db: &DatabaseConnection, question_id: usize) -> Result<bool> {
        Ok(<quiz::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<quiz::Entity as EntityTrait>::Column::Id.eq(question_id as i32)),
            )
            .count(db)
            .await
            .map_err(|err| {
                error!("Cannot fetch questions from DB. {}", err);
                Error::DatabaseError(format!("Cannot fetch questions from DB. {}", err))
            })?
            > 0)
    }

    pub async fn insert_questions(
        db: &DatabaseConnection,
        records: Vec<quiz::ActiveModel>,
    ) -> Result<()> {
        <quiz::Entity as EntityTrait>::insert_many(records)
            .exec(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot insert quiz. {}", err)))?;
        Ok(())
    }
}
