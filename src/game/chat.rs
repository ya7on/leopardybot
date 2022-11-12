use crate::entities::chat;
use crate::error::{Error, Result};
use crate::game::base::GameHandler;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

impl GameHandler {
    /// Register new chat if not registered yet. Returns bool (is new chat)
    pub async fn register_chat(db: &DatabaseConnection, chat_id: isize) -> Result<bool> {
        let chat_count = <chat::Entity as EntityTrait>::find()
            .filter(
                Condition::all().add(<chat::Entity as EntityTrait>::Column::Id.eq(chat_id as i64)),
            )
            .count(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot count chats. {}", err)))?;
        return if chat_count > 0 {
            Ok(false)
        } else {
            chat::ActiveModel {
                id: Set(chat_id as i64),
            }
            .insert(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot insert chat. {}", err)))?;
            Ok(true)
        };
    }
}
