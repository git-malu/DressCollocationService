use actix_web::{App, HttpServer};
use dress_collocation_service::app::{app_config, shared_date};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = Arc::new(shared_date());
    HttpServer::new(move || App::new().configure(|cfg| app_config(cfg, Arc::clone(&shared_data))))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
