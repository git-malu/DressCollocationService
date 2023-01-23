use actix_web::{App, HttpServer};
use dress_collocation_service::app::app_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(app_config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
