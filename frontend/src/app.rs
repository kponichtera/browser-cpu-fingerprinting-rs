use std::ops::Deref;
use gloo_console::info;
use common::dto::result::ResultDTO;

use gloo_net::http::Request;
use serde_json::{value::Value, Map};
use web_sys::HtmlInputElement;
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
    let model_input_ref = use_node_ref();
    let status_label_handle = use_state(String::default);
    let model_input_handle = use_state(String::default);
    let button_disabled_handle = use_state(|| true);
    let input_disabled_handle = use_state(|| false);

    let run_tests = {
        let status_label = status_label_handle.clone();
        let input_disabled_handle = input_disabled_handle.clone();
        let button_disabled_handle = button_disabled_handle.clone();
        let model_input_handle = model_input_handle.clone();
        Callback::from(move |_| {
            let status_label = status_label.clone();
            input_disabled_handle.set(true);
            button_disabled_handle.set(true);

            let (results, times): (Map<String, Value>, Map<String, Value>) = run_profilers(|profiler| {
                let status_label = status_label.clone();
                status_label.set(profiler.get_name().to_string());
            });
            let result = ResultDTO {
                model: model_input_handle.to_string(),
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

    let on_model_change = {
        let model_input_ref = model_input_ref.clone();
        let model_input_handle = model_input_handle.clone();
        let button_disabled_handle = button_disabled_handle.clone();

        Callback::from(move |_| {
            let input = model_input_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                model_input_handle.set(input.value());
                button_disabled_handle.set(input.value() == "");
            }
        })
    };

    let status_label = (*status_label_handle).clone();
    let model_input = (*model_input_handle).clone();
    let button_disabled = (*button_disabled_handle).clone();
    let input_disabled = (*input_disabled_handle).clone();

    html! {
        <main>
            <input id="model" ref={model_input_ref}
                value={model_input}
                oninput={on_model_change}
                disabled={input_disabled}/>
            <button onclick={run_tests} disabled={button_disabled}>{"Run tests"}</button>
            <p>{status_label}</p>
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
