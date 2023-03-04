use std::error::Error;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::api_server::config::ApiServerConfig;

#[derive(Clone)]
pub struct ApiServerContext {
    pub connection_pool: Pool<Postgres>,
}

pub async fn build_context(config: &ApiServerConfig) -> Result<ApiServerContext, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database_connection_count)
        .connect(config.database_url.as_str())
        .await?;

    Ok(ApiServerContext {
        connection_pool: pool,
    })
}