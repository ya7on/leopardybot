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
    pub async fn register_poll(
        &self,
        db: &DatabaseConnection,
        tg_poll: &Poll,
        message_id: usize,
    ) -> Result<()> {
        poll::ActiveModel {
            id: Set(tg_poll.id.clone()),
            message_id: Set(message_id as i64),
            game_id: Set(self.model.id as i32),
            correct_option_id: Set(tg_poll
                .correct_option_id
                .ok_or_else(|| Error("TODO".to_string()))?
                as i32),
            close_date: Set(tg_poll.close_date.map(|cd| cd as i32)),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(())
    }

    pub async fn get_unhandled_polls(db: &DatabaseConnection) -> Result<Vec<poll::Model>> {
        let now = Utc::now().timestamp();
        Ok(<poll::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<poll::Entity as EntityTrait>::Column::CloseDate.lt(now))
                    .add(<poll::Entity as EntityTrait>::Column::Handled.eq(false)),
            )
            .all(db)
            .await?)
    }

    pub async fn mark_poll_as_handled(db: &DatabaseConnection, poll_id: String) -> Result<()> {
        <poll::Entity as EntityTrait>::update_many()
            .filter(Condition::all().add(<poll::Entity as EntityTrait>::Column::Id.eq(poll_id)))
            .col_expr(
                <poll::Entity as EntityTrait>::Column::Handled,
                Expr::value(true),
            )
            .exec(db)
            .await?;
        Ok(())
    }

    pub async fn get_rounds(&self, db: &DatabaseConnection) -> Result<usize> {
        Ok(<poll::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<poll::Entity as EntityTrait>::Column::GameId.eq(self.model.id)),
            )
            .count(db)
            .await?)
    }

    pub async fn get_poll(db: &DatabaseConnection, poll_id: String) -> Result<poll::Model> {
        <poll::Entity as EntityTrait>::find()
            .filter(Condition::all().add(<poll::Entity as EntityTrait>::Column::Id.eq(poll_id)))
            .one(db)
            .await?
            .ok_or_else(|| Error("Cannot find poll".to_string()))
    }

    pub async fn get_active_polls(&self, db: &DatabaseConnection) -> Result<Vec<poll::Model>> {
        Ok(<poll::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<poll::Entity as EntityTrait>::Column::GameId.eq(self.model.id))
                    .add(<poll::Entity as EntityTrait>::Column::Handled.eq(false)),
            )
            .all(db)
            .await?)
    }
}
