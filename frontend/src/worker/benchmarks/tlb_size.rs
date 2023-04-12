use gloo_console::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::hint::black_box;
use std::mem::size_of;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

const PAGE_SIZE: usize = 4 * 1024;

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: usize,
    y: i64,
}

pub fn run_tlb_size_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running TLB size benchmark");
    let starting_time = clock.read();
    let entries = (2..126).step_by(4);
    let mut rand = rand::thread_rng();
    let result: Vec<DataPoint> = entries
        .into_iter()
        .map(|s| {
            let size = PAGE_SIZE * s / size_of::<usize>();
            let mut list = vec![0; size];
            let mut indices = (0..s).collect::<Vec<_>>();
            indices.shuffle(&mut rand);

            indices.windows(2).for_each(|w| list[w[0]] = w[1]);
            list[indices[s - 1]] = indices[0];

            let mut p = 0;

            for _ in 0..size {
                p = black_box(list[p]);
            }

            let start = clock.read();
            for _ in 0..size {
                p = black_box(list[p]);
            }
            let end = clock.read();

            info!(s, end - start);
            DataPoint {
                x: s,
                y: end - start,
            }
        })
        .collect::<Vec<_>>();

    BenchmarkResult {
        benchmark: BenchmarkType::TlbSize,
        result_json: json!(result).to_string(),
        time: (clock.read() - starting_time) as f32,
    }
}
