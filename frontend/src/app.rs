use common::dto::result::ResultDTO;

use gloo_net::http::Request;
use serde_json::json;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let response_indicator = use_state(|| "".to_string());
    let fire_some_data = {
        let response_indicator = response_indicator.clone();
        Callback::from(move |_| {
            let response_indicator = response_indicator.clone();
            let demo = ResultDTO {
                model: "foo".to_string(),
                user_agent: "bar".to_string(), // might require navigator via wasm_bindgen
                benchmark_results: json!(null),
                times: json!(null),
            };
            wasm_bindgen_futures::spawn_local(async move {
                response_indicator.set(
                    Request::post("/api/result/upload")
                        .json(&demo)
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
            <button onclick={fire_some_data}>{"Fire some data"}</button>
            <p>{&*response_indicator}</p>
        </main>
    }
}
