use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct TimerPrecisionProfiler;

impl Profiler for TimerPrecisionProfiler {

    fn get_name(&self) -> &'static str {
        "Timer precision"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running profiler:", self.get_name());
        (json!(null), json!(null))
    }

}