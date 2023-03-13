use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct L1dCacheAssociativityBenchmark;

impl Benchmark for L1dCacheAssociativityBenchmark {

    fn get_name(&self) -> &'static str {
        "L1D cache associativity"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}