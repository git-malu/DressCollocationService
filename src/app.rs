use crate::routes::hello_world::{echo, hello, manual_hello};

use actix_web::web;

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
}
