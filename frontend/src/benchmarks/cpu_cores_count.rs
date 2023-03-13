use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct CpuCoresCountBenchmark;

impl Benchmark for CpuCoresCountBenchmark {

    fn get_name(&self) -> &'static str {
        "CPU cores count"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}