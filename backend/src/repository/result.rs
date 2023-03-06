use std::error::Error;

use serde_json::json;
use sqlx::{Pool, Postgres};

use common::dto::result::ResultDTO;

pub async fn insert_result<'a>(
    pool: &Pool<Postgres>,
    result: &ResultDTO,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        "INSERT INTO upload_benchmarkresult (model, user_agent, benchmark_results, times, b64_charts)
        VALUES ($1, $2, $3, $4, $5)
        ",
        &result.model,
        &result.user_agent,
        &result.benchmark_results,
        &result.times,
        json!("[]")
    )
        .execute(pool)
        .await?;

    Ok(())
}
