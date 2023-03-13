use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct MemoryLatenciesProfiler;

impl Profiler for MemoryLatenciesProfiler {

    fn get_name(&self) -> &'static str {
        "Memory latencies"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running profiler:", self.get_name());
        (json!(null), json!(null))
    }

}