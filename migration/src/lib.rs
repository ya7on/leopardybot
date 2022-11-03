pub use sea_orm_migration::prelude::*;

mod m20220101_000001_player;
mod m20221026_195744_question;
mod m20221030_183111_chat;
mod m20221030_193210_game;
mod m20221031_205134_poll;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_player::Migration),
            Box::new(m20221026_195744_question::Migration),
            Box::new(m20221030_183111_chat::Migration),
            Box::new(m20221030_193210_game::Migration),
            Box::new(m20221031_205134_poll::Migration),
        ]
    }
}
