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
                    .table(Question::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Question::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Question::Text).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(QuestionOption::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QuestionOption::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QuestionOption::QuestionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-option-to-question")
                            .from(QuestionOption::Table, QuestionOption::QuestionId)
                            .to(Question::Table, Question::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(QuestionOption::Text).string().not_null())
                    .col(
                        ColumnDef::new(QuestionOption::IsCorrect)
                            .boolean()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(QuestionOption::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Question {
    Table,
    Id,
    Text,
}

#[derive(Iden)]
enum QuestionOption {
    Table,
    Id,
    QuestionId,
    Text,
    IsCorrect,
}
