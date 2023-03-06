use crate::context::BackendContext;
use crate::repository::result::insert_result;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};
use common::dto::result::ResultDTO;
use std::error::Error;

#[post("/api/result/upload")]
pub async fn upload(
    body: Json<ResultDTO>,
    context: Data<BackendContext>,
) -> Result<impl Responder, Box<dyn Error>> {
    insert_result(&context.connection_pool, &body).await?;

    Ok(HttpResponse::Ok())
}
