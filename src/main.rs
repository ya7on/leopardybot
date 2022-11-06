use leopardybot::conf::get_config;
use leopardybot::db::{clone_db, create_db};
use leopardybot::error::{Error, Result};
use leopardybot::telebot::client::Client;
use leopardybot::{job, seeder, server};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_module("sqlx::query", log::LevelFilter::Error)
        .init();

    let c = get_config();

    let db = create_db().await?;
    let client = Client::new(
        &c.telegram_token,
        &format!("https://{}/api/v1/telegram", &c.host),
        c.telegram_secret_token.as_ref(),
    )
    .await?;

    seeder::run(clone_db(&db)?).await?;
    job::run(clone_db(&db)?, client.clone()).await;
    server::run(db, client)
        .await
        .map_err(|err| Error::UncategorizedError(format!("Cannot run server. {}", err)))?;
    Ok(())
}
