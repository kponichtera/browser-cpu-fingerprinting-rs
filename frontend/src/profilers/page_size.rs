use crate::profilers::Profiler;
use gloo_console::info;
use serde_json::{json, value::Value};
use std::hint::black_box;

// Constants
const MB: usize = 1024 * 1024;
const MAXSIZE: usize = (MB + 100) / 8;

const START: usize = 512;

// const BUFFER_SIZE: usize = 64 * 1024 * 1024;
// const WASM_PAGE_SIZE: usize = 64 * 1024;
// const AMOUNT_OF_PAGES: usize  = BUFFER_SIZE / WASM_PAGE_SIZE;

pub struct PageSizeProfiler;

impl Profiler for PageSizeProfiler {
    fn get_name(&self) -> &'static str {
        "Page size"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());

        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        let mut buffer = [0; MAXSIZE];
        let mut size = 0;

        let mut results: Vec<f64> = Vec::new();

        while START + size * 4 < MAXSIZE {
            let start = performance.now();

            black_box(iteration(black_box(&mut buffer), black_box(size)));

            let end = performance.now();
            let diff = end - start;
            results.push(diff);
            size += 1;
        }

        for (index, data) in results.into_iter().enumerate() {
            if data != 0.0 {
                info!(format!("{}: {}", index, data));
            }
        }

        (json!(null), 0.0)
    }
}

fn iteration(buffer: &mut [i32; MAXSIZE], i: usize) {
    let _ = buffer[i];
}
