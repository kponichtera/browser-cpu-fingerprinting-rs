use actix_web::{HttpResponse, Responder, get};
use actix_web::web::Data;
use crate::context::BackendContext;

#[get("/api/misc/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok()
}