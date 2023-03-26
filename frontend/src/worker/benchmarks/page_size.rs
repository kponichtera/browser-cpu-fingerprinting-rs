use gloo_console::info;
use serde_json::json;
use std::hint::black_box;

use serde::Serialize;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

// Constants
const MB: usize = 1024 * 1024;
const MAXSIZE: usize = (MB + 100) / 8;

const START: usize = 512;

// const BUFFER_SIZE: usize = 64 * 1024 * 1024;
// const WASM_PAGE_SIZE: usize = 64 * 1024;
// const AMOUNT_OF_PAGES: usize  = BUFFER_SIZE / WASM_PAGE_SIZE;

#[derive(Serialize)]
struct DataPoint {
    x: i64,
    y: i64,
}

pub fn run_page_size_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running page size benchmark");

    let mut buffer = [0; MAXSIZE];
    let mut size = 0;

    let mut results: Vec<i64> = Vec::new();

    while START + size * 4 < MAXSIZE {
        let start = clock.read().unwrap();

        black_box(iteration(black_box(&mut buffer), black_box(size)));

        let end = clock.read().unwrap();

        let diff = end - start;
        results.push(diff);
        size += 1;
    }

    let mut result: Vec<DataPoint> = vec![];

    for (index, data) in results.into_iter().enumerate() {
        result.push(DataPoint {
            x: index as i64,
            y: data,
        });
        if data > 10 {
            info!(format!("{}: {}", index, data));
        }
    }

    BenchmarkResult {
        benchmark: BenchmarkType::PageSize,
        result_json: json!(result).to_string(),
        time: 0.0,
    }
}

fn iteration(buffer: &mut [i32; MAXSIZE], i: usize) {
    let _ = buffer[i];
}