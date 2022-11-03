use crate::extension::postgres::Type;
use crate::m20221030_183111_chat::Chat;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(GameModes::Type)
                    .values(vec![GameModes::Singleplayer, GameModes::Multiplayer])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::ChatId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-to-chat")
                            .from(Game::Table, Game::ChatId)
                            .to(Chat::Table, Chat::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Game::GameMode)
                            .custom(GameModes::Type)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::Active)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_type(Type::drop().name(GameModes::Type).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

enum GameModes {
    Type,
    Singleplayer,
    Multiplayer,
}

impl Iden for GameModes {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Type => "gamemodes",
                Self::Singleplayer => "singleplayer",
                Self::Multiplayer => "multiplayer",
            }
        )
        .unwrap();
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub(crate) enum Game {
    Table,
    Id,
    ChatId,
    Active,
    GameMode,
}
