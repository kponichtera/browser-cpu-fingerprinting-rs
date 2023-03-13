pub mod cache_associativity;
pub mod cache_size;
pub mod load_buffer_size;
pub mod memory_latencies;
pub mod multi_core_performance;
pub mod page_size;
pub mod prefetcher;
pub mod single_core_performance;
pub mod timer_precision;
pub mod tlb_size;

use serde_json::Value;

pub trait Profiler {

    fn get_name(&self) -> &'static str;

    fn run(&self) -> (Value, Value);

}