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
                    .table(Quiz::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Quiz::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Quiz::Text).string().not_null().unique_key())
                    .col(ColumnDef::new(Quiz::CorrectOption).string().not_null())
                    .col(ColumnDef::new(Quiz::Option2).string().not_null())
                    .col(ColumnDef::new(Quiz::Option3).string().not_null())
                    .col(ColumnDef::new(Quiz::Option4).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Quiz::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Quiz {
    Table,
    Id,
    Text,
    CorrectOption,
    Option2,
    Option3,
    Option4,
}
