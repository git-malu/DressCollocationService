use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn hello(data: Data<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {}", data.into_inner()))
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}