use common::dto::result::ResultDTO;

use gloo_net::http::Request;
use serde_json::{value::Value, Map};
use yew::prelude::*;
use crate::benchmarks::benchmark::Benchmark;
use crate::benchmarks::dummy::DummyBenchmark;

#[function_component(App)]
pub fn app() -> Html {
    let benchmarks: Vec<Box<dyn Benchmark>> = vec![
        Box::new(DummyBenchmark {})
    ];

    let response_indicator = use_state(|| String::from(""));
    let run_tests = {
        let response_indicator = response_indicator.clone();
        Callback::from(move |_| {
            let response_indicator = response_indicator.clone();
            let (results, times): (Map<String, Value>, Map<String, Value>) = benchmarks
                .iter()
                .map(|f| (f.get_name(), f.run()))
                .map(|(name, (results, times))| ((name.to_string(), results), (name.to_string(), times)))
                .unzip();
            let result = ResultDTO {
                model: "foo".to_string(),
                user_agent: get_user_agent().unwrap_or_else(|| "unknown".to_string()),
                benchmark_results: Value::Object(results),
                times: Value::Object(times),
            };
            wasm_bindgen_futures::spawn_local(async move {
                response_indicator.set(
                    Request::post("/api/result/upload")
                        .json(&result)
                        .unwrap()
                        .send()
                        .await
                        .unwrap()
                        .status()
                        .to_string(),
                );
            });
        })
    };

    html! {
        <main>
            <button onclick={run_tests}>{"Run tests"}</button>
            <p>{&*response_indicator}</p>
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
