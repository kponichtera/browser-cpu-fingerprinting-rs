use std::time::Duration;

use gloo_console::info;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};

use crate::profilers::polyfills::instant::Instant;
use crate::profilers::Profiler;

pub struct CacheSizeProfiler;

const KB: usize = 1024;

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: i16,
    y: Duration,
}

impl Profiler for CacheSizeProfiler {
    fn get_name(&self) -> &'static str {
        "Cache size"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());

        let mut sizes: Vec<usize> = vec![1 * KB];
        for i in (4..18).step_by(4) {
            sizes.push(i * KB);
        }

        // commented out to make benchmarking in the testing phase faster

        // sizes.push(1 * MB);
        //
        // for i in (2..33).step_by(2) {
        //     sizes.push(i * MB);
        // }

        let mut rand = rand::thread_rng();
        let result: Vec<DataPoint> = sizes
            .into_iter()
            .map(|s| {
                // this info may be removed at a later stage
                info!("Running cache-size profiler with size:", s);
                let size = KB * s / 8; // 8 bytes per usize
                let mut list = vec![0; size];
                let mut indices = (0..size).collect::<Vec<_>>();
                indices.shuffle(&mut rand);

                // the algorithm may need to be verified for correctness
                indices[1..].windows(2).for_each(|w| list[w[0]] = w[1]);
                list[indices[size - 1]] = indices[0];

                let start = Instant::now();

                let mut p = 0;
                for _ in 0..size {
                    p = std::hint::black_box(list[p]);
                }
                let end = Instant::now();
                DataPoint {
                    x: (size / KB) as i16,
                    y: end.duration_since(start),
                }
            })
            .collect::<Vec<_>>();
        (json!(result), 0.0)
    }
}
