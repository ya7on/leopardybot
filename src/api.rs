use crate::router::base::Router;
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use actix_web::web::Json;
use actix_web::{web, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;
use tracing::Level;

const TELEGRAM_SECRET_TOKEN_HEADER: &str = "X-Telegram-Bot-Api-Secret-Token";

fn verify_secret_token(request: HttpRequest, client: &Client) -> bool {
    if let Some(header) = request.headers().get(TELEGRAM_SECRET_TOKEN_HEADER) {
        if let Ok(header_value) = header.to_str() {
            return client.verify_secret_token(header_value);
        }
    }
    false
}

pub async fn handler(
    update: Json<Update>,
    db: web::Data<DatabaseConnection>,
    client: web::Data<Client>,
    router: web::Data<Router>,
    request: HttpRequest,
) -> HttpResponse {
    if !verify_secret_token(request, &client) {
        error!("Invalid secret token");
        return HttpResponse::Unauthorized().finish();
    }

    let span = span!(
        Level::DEBUG,
        "api",
        request_id = uuid::Uuid::new_v4().to_string()
    );
    let _enter = span.enter();

    debug!("Update: {:?}", update);

    if let Err(err) = router.handle(&update, &db, &client).await {
        error!("{:?}", err);
    }

    HttpResponse::Ok().finish()
}
