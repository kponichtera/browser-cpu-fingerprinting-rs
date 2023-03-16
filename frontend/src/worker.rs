use js_sys::{Atomics, BigInt64Array, SharedArrayBuffer};
use wasm_bindgen::JsValue;
use yew_agent::{WorkerLink, Worker, Public};

pub struct ClockWorker {
    // link: WorkerLink<Self>,
    clock: Clock,
}

impl ClockWorker {
    pub fn get_clock(&self) -> Clock {
        self.clock.clone()
    }
}

impl Worker for ClockWorker {
    type Reach = Public<Self>;

    type Message = Clock;

    type Input = ();

    type Output = ();

    fn create(link: WorkerLink<Self>) -> Self {
        let clock = Clock::new();
        link.send_message(clock.clone());
        link.respond(id, output)
    
        ClockWorker {
            // link,
            clock,
        }
    }

    fn update(&mut self, _msg: Self::Message) {
        // do nothing
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: yew_agent::HandlerId) {
        loop {
            self.clock.increment();
        }
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
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

    #[inline(always)]
    pub fn increment(&self) -> Result<(), JsValue> {
        Atomics::add_bigint(&self.data, 0, 1)?;
        Ok(())
    }

    #[inline(always)]
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
