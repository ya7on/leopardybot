use crate::api::handler;
use crate::conf::get_config;
use crate::db::clone_db;
use crate::router::base::Router;
use crate::telebot::client::Client;
use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;

pub async fn run(db: DatabaseConnection, client: Client, router: Router) -> std::io::Result<()> {
    let c = get_config();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/v1/telegram").to(handler))
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(clone_db(&db).unwrap()))
            .app_data(web::Data::new(router.clone()))
    })
    .bind(("0.0.0.0", c.port))?
    .workers(c.workers as usize)
    .run()
    .await
}
