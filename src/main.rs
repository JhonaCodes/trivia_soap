mod services;
mod models;

use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use crate::services::get_questions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let http_client = HttpClient::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(http_client.clone()))
            .route("/api/questions", web::get().to(get_questions))
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}