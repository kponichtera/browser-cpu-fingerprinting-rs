use gloo_console::info;
use serde_json::json;
use crate::worker::{BenchmarkResult, BenchmarkType};

pub fn run_dummy_benchmark() -> BenchmarkResult {
    info!("Running dummy benchmark");
    BenchmarkResult {
        benchmark: BenchmarkType::Dummy,
        result_json: json!(null).to_string(),
        time: 0.0
    }
}

