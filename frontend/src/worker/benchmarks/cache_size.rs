use std::hint::black_box;
use std::mem::size_of;

use gloo_console::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

const KB: usize = 1024;

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: u64,
    y: i64,
}

pub fn run_cache_size_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running cache size benchmark");
    let starting_time = clock.read().unwrap();
    let l0 = 1..=1;
    let l1 = (4..=512).step_by(4);
    let l2 = (1..=1).map(|x| x * 1024);
    let l3 = (2..=32).step_by(2).map(|x| x * 1024);

    let sizes: Vec<u64> = l0.chain(l1).chain(l2).chain(l3).collect();

    let mut rand = rand::thread_rng();
    let result: Vec<DataPoint> = sizes
        .into_iter()
        .map(|s| {
            // this info may be removed at a later stage
            info!("Running cache-size profiler with size:", s);
            let size = KB * s as usize / size_of::<usize>();
            let mut list = vec![0usize; size];
            let mut indices = (0..size).collect::<Vec<_>>();
            indices.shuffle(&mut rand);

            // the algorithm may need to be verified for correctness
            indices[1..].windows(2).for_each(|w| list[w[0]] = w[1]);
            list[indices[size - 1]] = indices[0];

            // warmup step; to make sure that the data is in the cache
            let mut p = 0;

            for _ in 0..size {
                // after we have established good data, check if these are necessary
                p = black_box(list[p]);
            }

            p = 0;
            let start = clock.read().unwrap();
            for _ in 0..size {
                p = black_box(list[p]);
            }
            let end = clock.read().unwrap();
            DataPoint { x: s * 1024, y: (end - start) / s as i64 }
        })
        .collect::<Vec<_>>();

    BenchmarkResult {
        benchmark: BenchmarkType::CacheSize,
        result_json: json!(result).to_string(),
        time: (clock.read().unwrap() - starting_time) as f32,
    }
}