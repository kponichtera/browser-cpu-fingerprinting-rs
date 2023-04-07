use std::mem::size_of;

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
    x: u16,
    y: u128,
}

impl Profiler for CacheSizeProfiler {
    fn get_name(&self) -> &'static str {
        "Cache size"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());

        let l1 = (10..150).step_by(10);
        let l2 = (150..2000).step_by(50);
        let l3 = (2000..25000).step_by(500);
        let sizes = [l1, l2, l3].map(|l| l.collect::<Vec<_>>()).concat();

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
                    p = std::hint::black_box(list[p]);
                }

                p = 0;
                let start = Instant::now();
                for _ in 0..size {
                    p = std::hint::black_box(list[p]);
                }
                let end = Instant::now();
                DataPoint {
                    x: s,
                    y: end.duration_since(start).as_nanos(),
                }
            })
            .collect::<Vec<_>>();
        (json!(result), 0.0)
    }
}
