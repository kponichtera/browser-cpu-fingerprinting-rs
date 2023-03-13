use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct LoadBufferSizeProfiler;

impl Profiler for LoadBufferSizeProfiler {

    fn get_name(&self) -> &'static str {
        "Load buffer profiler"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());
        (json!(null), 0.0)
    }

}