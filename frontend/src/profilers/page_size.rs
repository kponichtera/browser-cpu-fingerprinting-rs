use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct PageSizeProfiler;

impl Profiler for PageSizeProfiler {

    fn get_name(&self) -> &'static str {
        "Page size"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running profiler:", self.get_name());
        (json!(null), json!(null))
    }

}