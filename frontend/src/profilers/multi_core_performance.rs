use crate::profilers::Profiler;
use gloo_console::info;
use serde_json::{json, value::Value};

pub struct MultiCorePerformanceProfiler;

impl Profiler for MultiCorePerformanceProfiler {
    fn get_name(&self) -> &'static str {
        "Multi-core performance"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running benchmark:", self.get_name());
        (json!(null), 0.0)
    }
}
