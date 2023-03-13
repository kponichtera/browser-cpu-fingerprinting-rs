use std::ops::Deref;
use common::dto::result::ResultDTO;

use gloo_net::http::Request;
use serde_json::{value::Value, Map};
use yew::prelude::*;
use crate::profilers::Profiler;

use crate::profilers::cache_associativity::*;
use crate::profilers::cache_size::*;
use crate::profilers::load_buffer_size::*;
use crate::profilers::memory_latencies::*;
use crate::profilers::multi_core_performance::*;
use crate::profilers::page_size::*;
use crate::profilers::prefetcher::*;
use crate::profilers::single_core_performance::*;
use crate::profilers::timer_precision::*;
use crate::profilers::tlb_size::*;


#[function_component(App)]
pub fn app() -> Html {
    let status_label = use_state(|| String::from(""));
    
    let run_tests = {
        let status_label = status_label.clone();
        Callback::from(move |_| {
            let status_label = status_label.clone();
            let (results, times): (Map<String, Value>, Map<String, Value>) = run_profilers(|profiler| {
                let status_label = status_label.clone();
                status_label.set(profiler.get_name().to_string());
            });
            let result = ResultDTO {
                model: "foo".to_string(),
                user_agent: get_user_agent().unwrap_or_else(|| "unknown".to_string()),
                benchmark_results: Value::Object(results),
                times: Value::Object(times),
            };
            wasm_bindgen_futures::spawn_local(async move {
                status_label.set(
                    Request::post("/api/result/upload")
                        .json(&result)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                        .status_text()
                );
            });
        })
    };

    html! {
        <main>
            <button onclick={run_tests}>{"Run tests"}</button>
            <p>{&*status_label}</p>
        </main>
    }
}

fn get_user_agent() -> Option<String> {
    let window = web_sys::window().expect("Missing window");
    let user_agent = window.navigator().user_agent();
    match user_agent {
        Ok(user_agent) => Some(user_agent),
        Err(_) => None,
    }
}

fn run_profilers<T>(profiler_prehook: T) -> (Map<String, Value>, Map<String, Value>)
where T: FnOnce(&dyn Profiler) + Copy {
    let profilers: Vec<Box<dyn Profiler>> = vec![
        Box::new(PageSizeProfiler {}),
        Box::new(PrefetcherProfiler {}),
        Box::new(CacheAssociativityProfiler {}),
        Box::new(CacheSizeProfiler {}),
        Box::new(TlbSizeProfiler {}),
        Box::new(TimerPrecisionProfiler {}),
        Box::new(MemoryLatenciesProfiler {}),
        Box::new(LoadBufferSizeProfiler {}),
        Box::new(SingleCorePerformanceProfiler {}),
        Box::new(MultiCorePerformanceProfiler {}),
    ];

    let mut results = Map::new();
    let mut times = Map::new();

    for profiler in profilers {
        profiler_prehook(profiler.deref());
        let result = profiler.run();
        results.insert(profiler.get_name().to_string(), result.0);
        times.insert(profiler.get_name().to_string(), result.1);
    }

    (results, times)
}
