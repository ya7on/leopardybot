use crate::entities::poll;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use crate::telebot::typings::output::Poll;
use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

impl GameHandler {
    pub async fn register_poll(&self, db: &DatabaseConnection, tg_poll: &Poll) -> Result<()> {
        poll::ActiveModel {
            id: Set(tg_poll.id.clone()),
            game_id: Set(self.model.id as i32),
            correct_option_id: Set(tg_poll
                .correct_option_id
                .ok_or_else(|| Error::SerializationError("TODO".to_string()))?
                as i32),
            close_date: Set(Some(
                tg_poll
                    .close_date
                    .ok_or_else(|| Error::SerializationError("TODO".to_string()))?
                    as i32,
            )),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|err| Error::DatabaseError(format!("Cannot insert poll. {}", err)))?;
        Ok(())
    }

    pub async fn get_unhandled_polls(db: &DatabaseConnection) -> Result<Vec<poll::Model>> {
        let now = Utc::now().timestamp();
        <poll::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<poll::Entity as EntityTrait>::Column::CloseDate.lt(now))
                    .add(<poll::Entity as EntityTrait>::Column::Handled.eq(false)),
            )
            .all(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot fetch polls. {}", err)))
    }

    pub async fn mark_poll_as_handled(db: &DatabaseConnection, poll_id: String) -> Result<()> {
        <poll::Entity as EntityTrait>::update_many()
            .filter(Condition::all().add(<poll::Entity as EntityTrait>::Column::Id.eq(poll_id)))
            .col_expr(
                <poll::Entity as EntityTrait>::Column::Handled,
                Expr::value(true),
            )
            .exec(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot update poll. {}", err)))?;
        Ok(())
    }

    pub async fn get_rounds(&self, db: &DatabaseConnection) -> Result<usize> {
        <poll::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<poll::Entity as EntityTrait>::Column::GameId.eq(self.model.id)),
            )
            .count(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot update poll. {}", err)))
    }
}
