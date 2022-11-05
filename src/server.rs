use crate::api::handler;
use crate::conf::get_config;
use crate::db::clone_db;
use crate::telebot::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;

pub async fn run(db: DatabaseConnection, client: Client) -> std::io::Result<()> {
    let c = get_config();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/v1/telegram").to(handler))
            .app_data(client.clone())
            .app_data(clone_db(&db).unwrap())
    })
    .bind(("0.0.0.0", c.port))?
    .workers(c.workers)
    .run()
    .await
}
