use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}