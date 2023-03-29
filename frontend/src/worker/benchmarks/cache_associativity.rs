use std::hint::black_box;

use gloo_console::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

const MAX_SIZE: usize = 32;
const STEP: usize = 32 << 10;
const ITERATIONS: usize = 64 << 16;

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: u16,
    y: i64,
}

pub fn run_cache_associativity_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running cache associativity benchmark");
    let starting_time = clock.read().unwrap();
    let mut rand = rand::thread_rng();
    let result = (1..MAX_SIZE)
        .into_iter()
        .map(|s| {
            let size = STEP * s as usize;
            let mut list = vec![0; size];
            let mut indices = (0..size).step_by(STEP).collect::<Vec<_>>();
            indices.shuffle(&mut rand);
            indices.windows(2).for_each(|w| list[w[0]] = w[1]);
            list[indices[s - 1]] = indices[0];

            let mut p = 0;
            let start = clock.read().unwrap();
            for _ in 0..(s * ITERATIONS) {
                p = black_box(list[p]);
            }
            let end = clock.read().unwrap();
            DataPoint {
                x: s as u16,
                y: (end - start) / s as i64,
            }
        })
        .collect::<Vec<_>>();

    BenchmarkResult {
        benchmark: BenchmarkType::CacheAssociativity,
        result_json: json!(result).to_string(),
        time: (clock.read().unwrap() - starting_time) as f32,
    }
}
