use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::benchmark::Benchmark;

pub struct DummyBenchmark;

impl Benchmark for DummyBenchmark {

    fn get_name(&self) -> &'static str {
        "Dummy"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running dummy benchmark");
        (json!(null), json!(null))
    }

}