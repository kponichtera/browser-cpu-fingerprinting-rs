use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct SingleCorePerformanceBenchmark;

impl Benchmark for SingleCorePerformanceBenchmark {

    fn get_name(&self) -> &'static str {
        "Single core performance"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}