use common::dto::result::ResultDTO;
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use crate::profilers::Profiler;
use gloo_net::http::Request;
use serde_json::value::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

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
use crate::worker::{BenchmarkInput, BenchmarkResult, BenchmarkType, BenchmarkWorker};

pub enum AppRootMessage {
    ChangeModel(String),
    StartBenchmarks,
    BenchmarkComplete(BenchmarkResult),
    BenchmarksFinished(String),
}

pub struct AppRoot {
    bridge: Box<dyn Bridge<BenchmarkWorker>>,

    model_input: String,
    status_label: String,
    button_disabled: bool,
    input_disabled: bool,

    benchmark_results: Vec<BenchmarkResult>,
    remaining_benchmarks: VecDeque<BenchmarkType>,
}

impl Component for AppRoot {
    type Message = AppRootMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let worker_result_callback =
            move |result| link.send_message(AppRootMessage::BenchmarkComplete(result));

        AppRoot {
            bridge: BenchmarkWorker::bridge(Rc::new(worker_result_callback)),
            model_input: String::default(),
            status_label: String::default(),
            button_disabled: false,
            input_disabled: false,
            benchmark_results: Vec::new(),
            remaining_benchmarks: VecDeque::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppRootMessage::ChangeModel(new_model) => {
                self.model_input = new_model;
                true
            }
            AppRootMessage::StartBenchmarks => {
                self.start_benchmarks();
                true
            }
            AppRootMessage::BenchmarkComplete(result) => {
                self.handle_benchmark_complete(ctx, result);
                true
            }
            AppRootMessage::BenchmarksFinished(status) => {
                self.handle_benchmarks_finished(status);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let button_disabled = self.button_disabled || self.model_input.is_empty();

        html! {
            <main>
                <input id="model"
                    value={self.model_input.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                AppRootMessage::ChangeModel(input.value())
                            })}
                    disabled={self.input_disabled}/>
                <button onclick={ctx.link().callback(|_| { AppRootMessage::StartBenchmarks })}
                        disabled={button_disabled}>
                    { "Run tests" }
                </button>
                <p>{self.status_label.clone()}</p>
            </main>
        }
    }
}

impl AppRoot {
    fn start_benchmarks(&mut self) {
        self.button_disabled = true;
        self.input_disabled = true;
        self.benchmark_results = vec![];

        self.remaining_benchmarks = VecDeque::from(vec![
            // TODO: Add remaining benchmarks
            BenchmarkType::PageSize,
            BenchmarkType::CacheSize,
            BenchmarkType::SinglePerformance,
        ]);

        // Start with first benchmark
        let benchmark = self
            .remaining_benchmarks
            .pop_front()
            .expect("No benchmarks specified");
        self.status_label = benchmark.to_string();
        self.bridge.send(BenchmarkInput { benchmark });
    }

    fn handle_benchmark_complete(&mut self, ctx: &Context<Self>, result: BenchmarkResult) {
        self.benchmark_results.push(result);

        let next_benchmark = self.remaining_benchmarks.pop_front();
        match next_benchmark {
            Some(benchmark) => {
                // Run next benchmark
                self.status_label = benchmark.to_string();
                self.bridge.send(BenchmarkInput { benchmark });
            }
            None => {
                // No more benchmarks - send results to backend
                self.send_result(ctx);
            }
        }
    }

    fn send_result(&self, ctx: &Context<Self>) {
        let (results, times) = self.parse_results();

        let result = ResultDTO {
            model: self.model_input.clone(),
            user_agent: get_user_agent().unwrap_or_else(|| "unknown".to_string()),
            benchmark_results: results,
            times,
        };

        let link = ctx.link().clone();

        wasm_bindgen_futures::spawn_local(async move {
            let status = Request::post("/api/result/upload")
                .json(&result)
                .unwrap()
                .send()
                .await
                .unwrap()
                .status_text();

            link.send_message(AppRootMessage::BenchmarksFinished(status));
        });
    }

    fn handle_benchmarks_finished(&mut self, status: String) {
        self.button_disabled = false;
        self.input_disabled = false;
        self.status_label = status;
    }

    fn parse_results(&self) -> (Vec<Value>, Vec<f32>) {
        let mut results = vec![];
        let mut times = vec![];

        for result in self.benchmark_results.iter() {
            let value = serde_json::from_str::<Value>(result.result_json.clone().as_str()).unwrap();

            // TODO: Cloning the whole result JSON is not very optimal
            results.push(value);
            times.push(result.time);
        }

        (results, times)
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

/// TODO: For removal once benchmarks are fully handled by dedicated worker(s)
fn run_profilers<T>(profiler_prehook: T) -> (Vec<Value>, Vec<f32>)
where
    T: FnOnce(&dyn Profiler) + Copy,
{
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

    let mut results = vec![];
    let mut times = vec![];

    for profiler in profilers {
        profiler_prehook(profiler.deref());
        let result = profiler.run();
        results.push(result.0);
        times.push(result.1);
    }

    (results, times)
}
