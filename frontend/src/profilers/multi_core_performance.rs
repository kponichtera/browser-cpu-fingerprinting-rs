use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct MultiCorePerformanceProfiler;

impl Profiler for MultiCorePerformanceProfiler {

    fn get_name(&self) -> &'static str {
        "Multi-core performance"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running benchmark:", self.get_name());
        (json!(null), json!(null))
    }

}