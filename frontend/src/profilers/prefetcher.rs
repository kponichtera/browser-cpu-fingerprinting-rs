use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct PrefetcherProfiler;

impl Profiler for PrefetcherProfiler {

    fn get_name(&self) -> &'static str {
        "Prefetcher"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());
        (json!(null), 0.0)
    }

}