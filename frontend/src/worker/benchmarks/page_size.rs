use gloo_console::info;
use serde_json::json;
use std::hint::black_box;
use gloo_timers::callback::Timeout;
use js_sys::Atomics;
use web_sys::console::info;
use crate::worker::{BenchmarkResult, BenchmarkType};
use crate::worker::benchmarks::get_performance;
use crate::worker::clock::start_clock_worker;

// Constants
const MB: usize = 1024 * 1024;
const MAXSIZE: usize = (MB + 100) / 8;

const START: usize = 512;

// const BUFFER_SIZE: usize = 64 * 1024 * 1024;
// const WASM_PAGE_SIZE: usize = 64 * 1024;
// const AMOUNT_OF_PAGES: usize  = BUFFER_SIZE / WASM_PAGE_SIZE;

pub fn run_page_size_benchmark() -> BenchmarkResult {
    info!("Running page size benchmark");

    let (clock_worker, clock_data) = start_clock_worker().expect("clock worker should start");

    let mut buffer = [0; MAXSIZE];
    let mut size = 0;

    let mut results: Vec<i64> = Vec::new();

    while START + size * 4 < MAXSIZE {
        let start = Atomics::add_bigint(&clock_data, 0, 0)
            .expect("fetching clock value works");

        black_box(iteration(black_box(&mut buffer), black_box(size)));

        let end = Atomics::add_bigint(&clock_data, 0, 0)
            .expect("fetching clock value works");

        let diff = end - start;
        results.push(diff);
        size += 1;
    }

    for (index, data) in results.into_iter().enumerate() {
        // if data != 0 {
            info!(format!("{}: {}", index, data));
        // }
    }

    clock_worker.terminate();

    BenchmarkResult {
        benchmark: BenchmarkType::PageSize,
        result_json: json!(null).to_string(),
        time: 0.0
    }
}

fn iteration(buffer: &mut [i32; MAXSIZE], i: usize) {
    let _ = buffer[i];
}