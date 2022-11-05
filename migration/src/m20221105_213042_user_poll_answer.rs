use crate::m20220101_000001_player::Player;
use crate::m20221031_205134_poll::Poll;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(PlayerPollAnswer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayerPollAnswer::PlayerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-answer-to-player")
                            .from(PlayerPollAnswer::Table, PlayerPollAnswer::PlayerId)
                            .to(Player::Table, Player::TelegramId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(PlayerPollAnswer::PollId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-answer-to-poll")
                            .from(PlayerPollAnswer::Table, PlayerPollAnswer::PollId)
                            .to(Poll::Table, Poll::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(PlayerPollAnswer::IsCorrect)
                            .boolean()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("idx-answer-player-pollid")
                            .col(PlayerPollAnswer::PlayerId)
                            .col(PlayerPollAnswer::PollId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PlayerPollAnswer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PlayerPollAnswer {
    Table,
    PlayerId,
    PollId,
    IsCorrect,
}
