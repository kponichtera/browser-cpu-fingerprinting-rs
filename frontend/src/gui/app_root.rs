use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use gloo_net::http::Request;
use serde_json::value::Value;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_bootstrap::util::*;

use common::dto::result::ResultDTO;

use crate::gui::renders::*;
use crate::profilers::cache_associativity::*;
use crate::profilers::cache_size::*;
use crate::profilers::load_buffer_size::*;
use crate::profilers::memory_latencies::*;
use crate::profilers::multi_core_performance::*;
use crate::profilers::page_size::*;
use crate::profilers::prefetcher::*;
use crate::profilers::Profiler;
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
    total_benchmarks: usize,
    finished_benchmarks: usize,
    current_progress: f32,

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
            total_benchmarks: 0,
            finished_benchmarks: 0,
            current_progress: 0.0,
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
        <>
            {include_cdn()}
            {render_main_container(
                &self.model_input,
                self.input_disabled,
                &ctx,
                button_disabled,
                self.current_progress,
                &self.status_label,
            )}
            {include_cdn_js()}
            {render_footer()}
        </>
        }
    }
}

impl AppRoot {
    fn start_benchmarks(&mut self) {
        self.disable_controls();
        self.initialize_benchmark_data();

        self.start_next_benchmark_or_send(None);
    }

    fn disable_controls(&mut self) {
        self.button_disabled = true;
        self.input_disabled = true;
    }

    fn initialize_benchmark_data(&mut self) {
        self.benchmark_results = vec![];
        self.remaining_benchmarks = VecDeque::from(vec![
            // TODO: Add remaining benchmarks
            BenchmarkType::PageSize,
            BenchmarkType::CacheSize,
            BenchmarkType::TlbSize,
            BenchmarkType::SinglePerformance,
            BenchmarkType::CacheAssociativity,
        ]);

        self.total_benchmarks = self.remaining_benchmarks.len();
    }

    fn start_next_benchmark_or_send(&mut self, ctx: Option<&Context<Self>>) {
        if let Some(benchmark) = self.remaining_benchmarks.pop_front() {
            self.update_status_and_progress(benchmark);
            self.bridge.send(BenchmarkInput {
                page_origin: get_page_origin(),
                benchmark,
            });
        } else if let Some(ctx) = ctx {
            self.send_result(ctx);
        }
    }

    fn update_status_and_progress(&mut self, benchmark: BenchmarkType) {
        self.status_label = benchmark.to_string();
        self.finished_benchmarks += 1;
        self.current_progress = self.finished_benchmarks as f32 / self.total_benchmarks as f32 * 100.0;
    }

    fn handle_benchmark_complete(&mut self, ctx: &Context<Self>, result: BenchmarkResult) {
        self.benchmark_results.push(result);
        self.start_next_benchmark_or_send(Some(ctx));
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

fn get_page_origin() -> String {
    let window = web_sys::window().expect("Missing window");
    window.location()
        .origin()
        .expect("Missing origin information")
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
