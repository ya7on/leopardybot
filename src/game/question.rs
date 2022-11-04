use crate::entities::quiz;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::game::typings::{QuizPoll, QuizPollOption};
use rand::seq::SliceRandom;
use sea_orm::sea_query::ConditionExpression::SimpleExpr;
use sea_orm::sea_query::{Expr, Query};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, IntoSimpleExpr,
    Order, QueryFilter,
};

impl GameHandler {
    /// TODO FIXME удалить, когда будет реализован другой метод
    pub async fn get_question(db: &DatabaseConnection) -> Result<QuizPoll> {
        let stmt = Query::select()
            .from(quiz::Entity)
            .column(<quiz::Entity as EntityTrait>::Column::Id)
            .order_by_expr(Expr::cust("RANDOM()"), Order::Asc)
            .to_owned();
        let quiz = <quiz::Entity as EntityTrait>::find()
            .filter(
                Condition::all().add(<quiz::Entity as EntityTrait>::Column::Id.in_subquery(stmt)),
            )
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
            text: quiz.text.clone(),
            options,
            correct_answer_id,
        })
    }
}
