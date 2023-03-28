use std::hint::black_box;
use std::iter::StepBy;
use std::mem::size_of;
use std::ops::Range;

use gloo_console::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

const KB: usize = 1024;

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: usize,
    y: i64,
}

pub fn run_cache_size_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running cache size benchmark");
    let starting_time = clock.read().unwrap();
    let l1 = (4..512 + 1).step_by(4); // mimic the original sizes from the paper
    // let l2 = (150..2000).step_by(50);
    // let l3 = (2000..25000).step_by(500);
    // let sizes = [l1, l2, l3].map(|l| l.collect::<Vec<_>>()).concat();
    let mut sizes = [l1].map(|l| l.collect::<Vec<_>>()).concat();
    sizes.push(1); // 1 KB
    let mut rand = rand::thread_rng();
    let result: Vec<DataPoint> = sizes
        .into_iter()
        .map(|s| {
            // this info may be removed at a later stage
            info!("Running cache-size profiler with size:", s);
            let size = s * KB / size_of::<usize>();
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
                p = black_box(list[black_box(p)]);
            }
            let end = clock.read().unwrap();
            DataPoint { x: s * KB, y: (end - start) }
        })
        .collect::<Vec<_>>();

    BenchmarkResult {
        benchmark: BenchmarkType::CacheSize,
        result_json: json!(result).to_string(),
        time: (clock.read().unwrap() - starting_time) as f32,
    }
}