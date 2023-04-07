use crate::profilers::Profiler;
use gloo_console::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};
use wasm_bindgen::{closure::Closure, JsCast};

pub struct SingleCorePerformanceProfiler;

#[derive(Debug, Serialize, Deserialize)]
struct data_point {
    x: i16,
    y: i16,
}

impl Profiler for SingleCorePerformanceProfiler {
    fn get_name(&self) -> &'static str {
        "Single-core performance"
    }

    // "To estimate the single-core performance of the CPU, we increment a counter for the duration of 1 ms (measured using the performance.now function). We repeat this step for a ﬁxed number of iterations and collect the counter’s value after each iteration. To better observe the between boost and base frequency, we repeat this process three times and wait for 100 ms between each time to reset the frequency."

    fn run(&self) -> (Value, f32) {
        info!("Running profiler:", self.get_name());

        let iterations = 10; // should be 500 for production; 10 for quick testing now.
        let mut counter;
        let mut start;
        let mut data_array: Vec<data_point> = vec![];

        let log_delayed_info: Closure<dyn Fn()> = Closure::new(move || {
            info!("delay 100ms");
        });

        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        for i in 0..iterations {
            counter = 0;
            start = performance.now() + 1.0;
            while start > performance.now() {
                counter += 1;
            }
            info!(i, counter);
            data_array.push(data_point { x: i, y: counter });
        }

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                log_delayed_info.as_ref().unchecked_ref(),
                1000,
            )
            .expect("should register `setTimeout` OK");

        for i in 0..iterations {
            counter = 0;
            start = performance.now() + 1.0;
            while start > performance.now() {
                counter += 1;
            }
            info!(i, counter);
            data_array.push(data_point { x: i, y: counter });
        }

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                log_delayed_info.as_ref().unchecked_ref(),
                1000,
            )
            .expect("should register `setTimeout` OK");
        log_delayed_info.forget();

        for i in 0..iterations {
            counter = 0;
            start = performance.now() + 1.0;
            while start > performance.now() {
                counter += 1;
            }
            info!(i, counter);
            data_array.push(data_point { x: i, y: counter });
        }

        // returns json data, and the execution time (leave empty for now)
        (json!(data_array), 0.0)
    }
}
