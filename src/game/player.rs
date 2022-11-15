use crate::entities::player;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

impl GameHandler {
    pub async fn get_or_create_player(db: &DatabaseConnection, id: isize) -> Result<player::Model> {
        if let Some(player) = <player::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<player::Entity as EntityTrait>::Column::TelegramId.eq(id as i32)),
            )
            .one(db)
            .await?
        {
            Ok(player)
        } else {
            Ok(player::ActiveModel {
                telegram_id: Set(id as i32),
                ..Default::default()
            }
            .insert(db)
            .await?)
        }
    }

    pub async fn increase_player_score(
        db: &DatabaseConnection,
        player_id: isize,
        score: isize,
    ) -> Result<()> {
        let txn = db.begin().await?;
        let player = <player::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<player::Entity as EntityTrait>::Column::TelegramId.eq(player_id as i32)),
            )
            .one(&txn)
            .await?
            .ok_or_else(|| Error("Cannot find user model".to_string()))?;
        let old_score = player.score;
        let mut player_active_model: player::ActiveModel = player.into();
        player_active_model.score = Set(old_score + score as i32);
        player_active_model.update(&txn).await?;
        txn.commit().await?;
        Ok(())
    }
}
