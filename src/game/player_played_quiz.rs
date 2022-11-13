use crate::entities::player_played_quiz;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

impl GameHandler {
    pub async fn mark_quiz_as_played(
        db: &DatabaseConnection,
        player_id: isize,
        quiz_id: isize,
    ) -> Result<()> {
        player_played_quiz::ActiveModel {
            player_id: Set(player_id as i32),
            quiz_id: Set(quiz_id as i32),
        }
        .insert(db)
        .await
        .map_err(|err| {
            Error::DatabaseError(format!("Cannot insert player_played_quiz. {}", err))
        })?;
        Ok(())
    }
}
