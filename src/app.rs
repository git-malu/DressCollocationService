use crate::routes::hello_world::{echo, hello, manual_hello};
use std::sync::Arc;

use actix_web::web;
use actix_web::web::Data;

pub fn app_config(cfg: &mut web::ServiceConfig, shared_data: Arc<SharedData>) {
    cfg.service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .app_data(Data::clone(&shared_data.count));
}

#[derive(Clone)]
pub struct SharedData {
    count: Data<i32>,
}

pub fn shared_date() -> SharedData {
    return SharedData {
        count: Data::new(0),
    };
}
