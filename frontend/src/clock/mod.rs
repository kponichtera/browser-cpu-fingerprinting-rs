use js_sys::{Atomics, BigInt64Array, SharedArrayBuffer};

pub const CLOCK_MESSAGE_READY: &str = "clock_ready";
pub const CLOCK_MESSAGE_STARTED: &str = "clock_started";

/// Clock implementation using SharedArrayBuffer. Based on
/// [wasm-rs-shared-channel](https://docs.rs/wasm-rs-shared-channel/0.1.0/src/wasm_rs_shared_channel/spsc.rs.html#128-135)
pub struct Clock {
    pub shared_buffer: SharedArrayBuffer,
    pub data: BigInt64Array,
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

    #[inline(always)]
    pub fn increment(&self) {
        let _ = Atomics::add_bigint(&self.data, 0, 1);
    }

    #[inline(always)]
    pub fn read(&self) -> i64 {
        self.data.get_index(0)
    }
}

impl From<SharedArrayBuffer> for Clock {
    fn from(value: SharedArrayBuffer) -> Self {
        let data = BigInt64Array::new(&value);
        Clock {
            shared_buffer: value,
            data,
        }
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
