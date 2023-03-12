use crate::benchmarks::dummy;
use common::dto::result::ResultDTO;

use gloo_net::http::Request;
use serde_json::{value::Value, Map};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let benchmarks = vec![dummy::dummy_benchmark];

    let response_indicator = use_state(|| String::from(""));
    let run_tests = {
        let response_indicator = response_indicator.clone();
        Callback::from(move |_| {
            let response_indicator = response_indicator.clone();
            let (results, times): (Map<String, Value>, Map<String, Value>) = benchmarks
                .iter()
                .map(|f| f())
                .map(|(name, results, times)| ((name.clone(), results), (name, times)))
                .unzip();
            let result = ResultDTO {
                model: "foo".to_string(),
                user_agent: "bar".to_string(), // might require navigator via wasm_bindgen
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
