use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}