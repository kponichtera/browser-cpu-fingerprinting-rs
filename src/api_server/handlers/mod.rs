use actix_web::{web};

mod misc;
mod result;

pub fn register_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(misc::ping);
}
