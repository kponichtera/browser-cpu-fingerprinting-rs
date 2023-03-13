use crate::profilers::Profiler;
use gloo_console::info;
use serde_json::{json, value::Value};

pub struct TimerPrecisionProfiler;

impl Profiler for TimerPrecisionProfiler {
    fn get_name(&self) -> &'static str {
        "Timer precision"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());
        (json!(null), 0.0)
    }
}
