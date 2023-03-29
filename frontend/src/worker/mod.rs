use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Private, Worker, WorkerLink};

use crate::clock::Clock;
use crate::worker::benchmarks::tlb_size::run_tlb_size_benchmark;
use crate::worker::benchmarks::cache_size::run_cache_size_benchmark;
use crate::worker::benchmarks::page_size::run_page_size_benchmark;
use crate::worker::clock::start_clock_worker;

mod benchmarks;
mod clock;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum BenchmarkType {
    PageSize,
    CacheSize,
    TlbSize,
}

impl Display for BenchmarkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkType::PageSize => write!(f, "Page size"),
            BenchmarkType::CacheSize => write!(f, "Cache size"),
            BenchmarkType::TlbSize => write!(f, "TLB size"),
        }
    }
}

impl BenchmarkType {
    fn needs_clock(&self) -> bool {
        match self {
            BenchmarkType::PageSize => true,
            BenchmarkType::CacheSize => true,
            BenchmarkType::TlbSize => true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BenchmarkInput {
    pub benchmark: BenchmarkType,
    /// Origin of the webpage, required by the spawned workers to load the scripts
    pub page_origin: String,
}

#[derive(Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub benchmark: BenchmarkType,
    /// Workaround for broken (de)serialization of raw JSON value.
    /// Turn to proper serde_json::Value after receiving.
    pub result_json: String,
    pub time: f32,
}

pub struct BenchmarkWorker {
    link: WorkerLink<Self>,
}

impl Worker for BenchmarkWorker {
    type Reach = Private<Self>;
    type Message = ();
    type Input = BenchmarkInput;
    type Output = BenchmarkResult;

    fn create(link: WorkerLink<Self>) -> Self {
        BenchmarkWorker { link }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        if msg.benchmark.needs_clock() {
            let link = self.link.clone();
            // start the clock and run benchmark in the callback
            start_clock_worker(msg.page_origin, move |clock, clock_worker| {
                let result = run_benchmark(msg.benchmark, Some(clock));
                clock_worker.terminate();
                link.respond(id, result);
            })
            .expect("clock worker should start");
        } else {
            // run benchmark directly
            let result = run_benchmark(msg.benchmark, None);
            self.link.respond(id, result);
        }
    }

    fn name_of_resource() -> &'static str {
        "benchmark_worker.js"
    }
}

fn run_benchmark(benchmark: BenchmarkType, clock: Option<Clock>) -> BenchmarkResult {
    match benchmark {
        BenchmarkType::PageSize => run_page_size_benchmark(clock.unwrap()),
        BenchmarkType::CacheSize => run_cache_size_benchmark(clock.unwrap()),
        BenchmarkType::TlbSize => run_tlb_size_benchmark(clock.unwrap()),
    }
}
