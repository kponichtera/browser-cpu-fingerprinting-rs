use gloo_console::info;
use serde_json::json;
use std::hint::black_box;

use serde::Serialize;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

// Constants
const MB: usize = 1024 * 1024;
const START: usize = 512;
const MAXSIZE: usize = MB + 100;

#[derive(Serialize)]
struct DataPoint {
    x: usize,
    y: i64,
}

pub fn run_page_size_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running page size benchmark");

    let mut buffer = [0; MAXSIZE];
    let mut size = 0;

    let mut results: Vec<DataPoint> = Vec::new();

    while START + size * 4 < MAXSIZE {
        let diff = black_box(iteration(&clock, &mut buffer, black_box(START + size * 4)));

        results.push(DataPoint {
            x: START + size * 4,
            y: diff,
        });
        size += 1;
    }

    BenchmarkResult {
        benchmark: BenchmarkType::PageSize,
        result_json: json!(results).to_string(),
        time: 0.0,
    }
}

#[allow(unused_variables, unused_assignments)]
fn iteration(clock: &Clock, buffer: &mut [i32; MAXSIZE], i: usize) -> i64 {
    let start = clock.read();

    let mut tmp = 0;
    tmp = unsafe { std::ptr::read(buffer.as_mut_ptr().add(i)) };

    let end = clock.read();
    end - start
}
