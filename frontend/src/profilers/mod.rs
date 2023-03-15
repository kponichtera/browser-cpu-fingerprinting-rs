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

use js_sys::{BigInt64Array, SharedArrayBuffer, Atomics};
use serde_json::Value;
use wasm_bindgen::JsValue;

pub trait Profiler {
    fn get_name(&self) -> &'static str;

    fn run(&self) -> (Value, f32);
}

/// Clock implementation using SharedArrayBuffer. Based on
/// [wasm-rs-shared-channel](https://docs.rs/wasm-rs-shared-channel/0.1.0/src/wasm_rs_shared_channel/spsc.rs.html#128-135)
pub struct Clock {
    shared_buffer: SharedArrayBuffer,
    data: BigInt64Array,
}

impl Clock {
    pub fn new() -> Clock {
        let shared_buffer = SharedArrayBuffer::new(std::mem::size_of::<i64>() as u32);
        let data = BigInt64Array::new(&shared_buffer);

        Atomics::store_bigint(&data, 0, 0).unwrap();

        Clock {
            shared_buffer,
            data,
        }
    }

    pub fn increment(&self) -> Result<(), JsValue> {
        Atomics::add_bigint(&self.data, 0, 1)?;
        Ok(())
    }

    pub fn read(&self) -> Result<i64, JsValue> {
        // Currently using add with zero, load_bigint seems to give a JS error.
        let t = Atomics::add_bigint(&self.data, 0, 0)?;
        Ok(t)
    }
}

impl Clone for Clock {
    fn clone(&self) -> Self {
        Clock {
            shared_buffer: self.shared_buffer.clone(),
            data: self.data.clone(),
        }
    }
}
