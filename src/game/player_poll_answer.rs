use crate::entities::player_poll_answer;
use crate::error::Result;
use crate::game::base::GameHandler;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

impl GameHandler {
    pub async fn add_user_poll_answer(
        db: &DatabaseConnection,
        user_id: isize,
        poll_id: String,
        is_correct: bool,
    ) -> Result<()> {
        player_poll_answer::ActiveModel {
            player_id: Set(user_id as i32),
            poll_id: Set(poll_id),
            is_correct: Set(is_correct),
        }
        .insert(db)
        .await?;
        Ok(())
    }
}
