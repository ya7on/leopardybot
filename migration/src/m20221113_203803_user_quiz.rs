use crate::idens::{Player, PlayerPlayedQuiz, Quiz};
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
                    .table(PlayerPlayedQuiz::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayerPlayedQuiz::PlayerId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-played-quiz-to-player")
                            .from(PlayerPlayedQuiz::Table, PlayerPlayedQuiz::PlayerId)
                            .to(Player::Table, Player::TelegramId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(PlayerPlayedQuiz::QuizId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-played-quiz-to-quiz")
                            .from(PlayerPlayedQuiz::Table, PlayerPlayedQuiz::QuizId)
                            .to(Quiz::Table, Quiz::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .name("idx-player-quizid-played")
                            .col(PlayerPlayedQuiz::PlayerId)
                            .col(PlayerPlayedQuiz::QuizId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(PlayerPlayedQuiz::Table).to_owned())
            .await
    }
}
