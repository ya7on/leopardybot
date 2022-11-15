use crate::entities::quiz;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::game::typings::{QuizPoll, QuizPollOption};
use rand::seq::SliceRandom;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, Statement};

impl GameHandler {
    fn parse_question_result(quiz_model: quiz::Model) -> Result<QuizPoll> {
        let mut options = vec![
            QuizPollOption {
                is_correct: true,
                text: quiz_model.correct_option,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz_model.option2,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz_model.option3,
            },
            QuizPollOption {
                is_correct: false,
                text: quiz_model.option4,
            },
        ];
        options.shuffle(&mut rand::thread_rng());

        let correct_answer_id = options
            .iter()
            .position(|i| i.is_correct)
            .ok_or_else(|| Error("Cannot find correct answer".to_string()))?;

        Ok(QuizPoll {
            id: quiz_model.id,
            text: quiz_model.text,
            options,
            correct_answer_id,
        })
    }

    pub async fn get_new_question(
        db: &DatabaseConnection,
        player_id: isize,
    ) -> Result<Option<QuizPoll>> {
        let sql = "SELECT q.* FROM public.quiz q WHERE q.id NOT IN (SELECT DISTINCT ppq.quiz_id FROM public.player_played_quiz ppq WHERE ppq.player_id = $1) ORDER BY RANDOM() LIMIT 1;";
        let stmt = Statement::from_sql_and_values(
            db.get_database_backend(),
            sql,
            vec![(player_id as i32).into()],
        );
        if let Some(quiz) = <quiz::Entity as EntityTrait>::find()
            .from_raw_sql(stmt)
            .one(db)
            .await?
        {
            Ok(Some(Self::parse_question_result(quiz)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_question(db: &DatabaseConnection) -> Result<QuizPoll> {
        let sql = "SELECT * FROM public.quiz ORDER BY RANDOM() LIMIT 1;";
        let stmt = Statement::from_sql_and_values(db.get_database_backend(), sql, vec![]);
        let quiz = <quiz::Entity as EntityTrait>::find()
            .from_raw_sql(stmt)
            .one(db)
            .await?
            .ok_or_else(|| Error("Empty question result".to_string()))?;
        Self::parse_question_result(quiz)
    }

    // pub async fn clear_question(db: &DatabaseConnection) -> Result<()> {
    //     <quiz::Entity as EntityTrait>::delete_many()
    //         .exec(db)
    //         .await
    //         .map_err(|err| Error::DatabaseError(format!("Cannot delete quiz. {}", err)))?;
    //     Ok(())
    // }

    // pub async fn question_exists(db: &DatabaseConnection, question_id: usize) -> Result<bool> {
    //     Ok(<quiz::Entity as EntityTrait>::find()
    //         .filter(
    //             Condition::all()
    //                 .add(<quiz::Entity as EntityTrait>::Column::Id.eq(question_id as i32)),
    //         )
    //         .count(db)
    //         .await
    //         .map_err(|err| {
    //             error!("Cannot fetch questions from DB. {}", err);
    //             Error::DatabaseError(format!("Cannot fetch questions from DB. {}", err))
    //         })?
    //         > 0)
    // }

    pub async fn insert_questions(
        db: &DatabaseConnection,
        records: Vec<quiz::ActiveModel>,
    ) -> Result<()> {
        <quiz::Entity as EntityTrait>::insert_many(records)
            .on_conflict(
                OnConflict::column(<quiz::Entity as EntityTrait>::Column::Id)
                    .update_columns(vec![
                        <quiz::Entity as EntityTrait>::Column::Text,
                        <quiz::Entity as EntityTrait>::Column::CorrectOption,
                        <quiz::Entity as EntityTrait>::Column::Option2,
                        <quiz::Entity as EntityTrait>::Column::Option3,
                        <quiz::Entity as EntityTrait>::Column::Option4,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;
        Ok(())
    }
}
