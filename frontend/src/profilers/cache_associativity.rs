use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct CacheAssociativityProfiler;

impl Profiler for CacheAssociativityProfiler {

    fn get_name(&self) -> &'static str {
        "Cache associativity"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());
        (json!(null), 0.0)
    }

}