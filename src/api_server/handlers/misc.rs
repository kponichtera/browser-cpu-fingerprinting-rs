use actix_web::{HttpResponse, Responder, get};

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok()
}