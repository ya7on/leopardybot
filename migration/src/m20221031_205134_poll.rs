use crate::m20221030_193210_game::Game;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Poll::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Poll::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Poll::MessageId).big_integer().not_null())
                    .col(ColumnDef::new(Poll::GameId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-poll-to-game")
                            .from(Poll::Table, Poll::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Poll::CorrectOptionId).integer().not_null())
                    .col(ColumnDef::new(Poll::CloseDate).integer())
                    .col(ColumnDef::new(Poll::Handled).boolean().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Poll::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Poll {
    Table,
    Id,
    MessageId,
    GameId,
    CorrectOptionId,
    CloseDate,
    Handled,
}
