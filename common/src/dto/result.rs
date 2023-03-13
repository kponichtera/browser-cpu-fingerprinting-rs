use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResultDTO {
    pub model: String,
    pub user_agent: String,
    pub benchmark_results: Vec<serde_json::Value>,
    pub times: Vec<f32>,
}
