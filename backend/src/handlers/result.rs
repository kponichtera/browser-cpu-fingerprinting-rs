use std::error::Error;
use actix_web::{post, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use common::dto::result::ResultDTO;
use crate::context::BackendContext;
use crate::repository::result::insert_result;

#[post("/api/result/upload")]
pub async fn upload(body: Json<ResultDTO>, context: Data<BackendContext>) -> impl Responder {
    let insertion_result = insert_result(&context.connection_pool, &body).await;

    match insertion_result {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }

}