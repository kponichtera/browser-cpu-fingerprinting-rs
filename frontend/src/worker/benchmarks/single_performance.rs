use gloo_console::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::clock::Clock;
use crate::worker::{BenchmarkResult, BenchmarkType};

#[derive(Debug, Serialize, Deserialize)]
struct DataPoint {
    x: i16,
    y: i16,
}

pub fn run_single_performance_benchmark(clock: Clock) -> BenchmarkResult {
    info!("Running single core performance benchmark");

    let total_starting_time = clock.read();

    let iterations = 500; // set 10 for quick testing
    let mut counter;
    let mut end;
    let mut data_array: Vec<DataPoint> = vec![];

    info!("Single core: first iteration");
    for i in 0..iterations {
        counter = 0;
        end = clock.read() + 1000;
        while end > clock.read() {
            counter += 1;
        }
        data_array.push(DataPoint {
            x: (i),
            y: (counter),
        });
    }

    info!("Single core: second iteration");
    for i in iterations..iterations * 2 {
        counter = 0;
        end = clock.read() + 1000;
        while end > clock.read() {
            counter += 1;
        }
        data_array.push(DataPoint {
            x: (i),
            y: (counter),
        });
    }

    info!("Single core: third iteration");
    for i in iterations * 2..iterations * 3 {
        counter = 0;
        end = clock.read() + 1000;
        while end > clock.read() {
            counter += 1;
        }
        data_array.push(DataPoint {
            x: (i),
            y: (counter),
        });
    }

    BenchmarkResult {
        benchmark: BenchmarkType::SinglePerformance,
        result_json: json!(data_array).to_string(),
        time: (clock.read() - total_starting_time) as f32,
    }
}
