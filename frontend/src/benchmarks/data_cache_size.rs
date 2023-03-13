use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct DataCacheSizeBenchmark;

impl Benchmark for DataCacheSizeBenchmark {

    fn get_name(&self) -> &'static str {
        "Data cache size"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}