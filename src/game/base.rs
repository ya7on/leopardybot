use crate::entities::game;
use crate::entities::sea_orm_active_enums::Gamemodes;
use crate::error::{Error, Result};
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

pub struct GameHandler {
    pub model: game::Model,
}

impl GameHandler {
    pub async fn exists(db: &DatabaseConnection, chat_id: isize) -> Result<bool> {
        let count = <game::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<game::Entity as EntityTrait>::Column::ChatId.eq(chat_id as i32))
                    .add(<game::Entity as EntityTrait>::Column::Active.eq(true)),
            )
            .count(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot count games. {}", err)))?;
        Ok(count > 0)
    }

    /// Create new game with chat id
    pub async fn create(db: &DatabaseConnection, chat_id: isize) -> Result<Self> {
        let g = game::ActiveModel {
            chat_id: Set(chat_id as i32),
            game_mode: Set(Gamemodes::Singleplayer),
            ..Default::default()
        }
        .insert(db)
        .await
        .map_err(|err| Error::DatabaseError(format!("Cannot insert game. {}", err)))?;
        Ok(Self { model: g })
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: usize) -> Result<Self> {
        let g = <game::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<game::Entity as EntityTrait>::Column::Id.eq(id as i32))
                    .add(<game::Entity as EntityTrait>::Column::Active.eq(true)),
            )
            .one(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot count games. {}", err)))?
            .ok_or_else(|| Error::DatabaseError(format!("Cannot fetch game with id {}", id)))?;
        Ok(Self { model: g })
    }

    /// Get game instance with chat id
    pub async fn get_by_chat_id(db: &DatabaseConnection, chat_id: isize) -> Result<Self> {
        let g = <game::Entity as EntityTrait>::find()
            .filter(
                Condition::all()
                    .add(<game::Entity as EntityTrait>::Column::ChatId.eq(chat_id as i32))
                    .add(<game::Entity as EntityTrait>::Column::Active.eq(true)),
            )
            .one(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot count games. {}", err)))?
            .ok_or_else(|| {
                Error::DatabaseError(format!("Cannot fetch game with chat id {}", chat_id))
            })?;

        Ok(Self { model: g })
    }

    pub async fn end_game(&self, db: &DatabaseConnection) -> Result<()> {
        <game::Entity as EntityTrait>::update_many()
            .filter(
                Condition::all()
                    .add(<game::Entity as EntityTrait>::Column::Id.eq(self.model.id as i32)),
            )
            .col_expr(
                <game::Entity as EntityTrait>::Column::Active,
                Expr::value(false),
            )
            .exec(db)
            .await
            .map_err(|err| Error::DatabaseError(format!("Cannot update game. {}", err)))?;
        Ok(())
    }

    // /// TODO Find question that users haven't player before
    // async fn find_new_question() -> Result<QuizPoll> {
    //     todo!()
    // }
    // /// TODO Find any question (if it's not new questions)
    // async fn find_any_question() -> Result<QuizPoll> {
    //     todo!()
    // }
    // /// TODO Find question for players
    // pub async fn find_question() -> Result<QuizPoll> {
    //     todo!()
    // }
    // /// TODO register player's answer in poll
    // pub async fn register_answer() -> Result<()> {
    //     todo!()
    // }
    // /// TODO register new player if it's new
    // pub async fn register_player() -> Result<bool> {
    //     todo!()
    // }
    // /// TODO summarize game, update player's score
    // pub async fn end_game() -> Result<()> {
    //     todo!()
    // }
}
