use gloo_console::info;
use serde_json::{json, value::Value};
use crate::profilers::Profiler;

pub struct TlbSizeProfiler;

impl Profiler for TlbSizeProfiler {

    fn get_name(&self) -> &'static str {
        "TLB size"
    }

    fn run(&self) -> (Value, Value) {
        info!("Running profiler:", self.get_name());
        (json!(null), json!(null))
    }

}