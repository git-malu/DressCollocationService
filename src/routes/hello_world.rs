use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn index(data: Data<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {}", data.into_inner()))
}
