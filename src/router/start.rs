use crate::error::Result;
use crate::router::base::RouteHandler;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::texts::TextFormatter;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct StartCommand;

#[async_trait::async_trait]
impl RouteHandler for StartCommand {
    async fn handle(&self, _: &DatabaseConnection, client: &Client, update: &Update) -> Result<()> {
        if let Some(message) = &update.message {
            client
                .send_message(message.chat.id, &TextFormatter::start()?)
                .await?;
        }
        Ok(())
    }
}
