use actix_web::{get, HttpResponse, Responder};

#[get("/api/misc/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok()
}
