pub mod l1d_tlb_size;
pub mod page_size;
pub mod cpu_cores_count;
pub mod l1d_cache_associativity;
pub mod single_core_performance;
pub mod data_cache_size;


use serde_json::Value;

pub trait Benchmark {

    fn get_name(&self) -> &'static str;

    fn run(&self) -> (Value, Value);

}