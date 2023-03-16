use crate::profilers::Profiler;
use gloo_console::info;
use rand::seq::SliceRandom;
use serde_json::{json, value::Value};

const MAX_SIZE: usize = 32;
const KB: usize = 1024;

pub struct CacheAssociativityProfiler;

impl Profiler for CacheAssociativityProfiler {
    fn get_name(&self) -> &'static str {
        "Cache associativity"
    }

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());

        let mut rand = rand::thread_rng();
        let result = (1..MAX_SIZE)
            .into_iter()
            .map(|s| {
                let size = 32 * KB * s;
                let mut list = vec![0; size];
                let mut indices = (0..(32 * KB * size)).collect::<Vec<_>>();
                indices.shuffle(&mut rand);
                indices[1..].windows(2).for_each(|w| list[w[0]] = w[1]);
                list[indices[size - 1]] = indices[0];

                let mut p = 0;
                for _ in 0..size {
                    p = std::hint::black_box(list[p]);
                }
                0 // TODO timing
            })
            .collect::<Vec<_>>();

        (json!(result), 0.0)
    }
}
