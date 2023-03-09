use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResultDTO {
    pub model: String,
    pub user_agent: String,
    pub benchmark_results: serde_json::Value,
    pub times: serde_json::Value,
}
