use std::sync::Arc;

use crate::routes::echo::echo;
use crate::routes::hello_world::index;
use crate::routes::manual_hello::manual_hello;
use actix_web::web;
use actix_web::web::Data;

pub fn app_config(cfg: &mut web::ServiceConfig, shared_data: Arc<SharedData>) {
    cfg.service(index)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .app_data(Data::clone(&shared_data.count));
}

#[derive(Clone)]
pub struct SharedData {
    count: Data<i32>,
}

pub fn shared_data() -> SharedData {
    return SharedData {
        count: Data::new(0),
    };
}
