use gloo_console::info;
use serde_json::{json, value::Value};
use crate::benchmarks::Benchmark;

pub struct L1dTlbSizeBenchmark;

impl Benchmark for L1dTlbSizeBenchmark {

    fn get_name(&self) -> &'static str {
        "L1D TLB size"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}