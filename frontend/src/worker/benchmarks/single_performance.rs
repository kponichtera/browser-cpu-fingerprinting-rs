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

    let iterations = 500; // should be 500 for production; 10 for quick testing now.
    let mut counter;
    let mut end;
    let mut data_array: Vec<DataPoint> = vec![];

    /*
    let log_delayed_info: Closure<dyn Fn()> = Closure::new(move || {
        info!("delay 100ms");
    });

    let window = web_sys::window().expect("should have a window in this context");
        let performance = window
        .performance()
        .expect("performance should be available");

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(log_delayed_info.as_ref().unchecked_ref(),1000)
        .expect("should register `setTimeout` OK");
    */

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
