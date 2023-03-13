use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct PageSizeBenchmark;

impl Benchmark for PageSizeBenchmark {

    fn get_name(&self) -> &'static str {
        "Page size"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}