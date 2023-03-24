mod benchmarks;

use std::fmt::{Display, Formatter};
use yew_agent::{HandlerId, Private, Public, Worker, WorkerLink};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::worker::benchmarks::dummy::run_dummy_benchmark;

#[derive(Serialize, Deserialize)]
pub enum BenchmarkType {
    Dummy
}

impl Display for BenchmarkType {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkType::Dummy => write!(f, "Dummy")
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct BenchmarkInput {
    pub benchmark: BenchmarkType
}

#[derive(Serialize, Deserialize)]
pub struct BenchmarkResult {
    benchmark: BenchmarkType,
    /// Workaround for broken (de)serialization of raw JSON value.
    /// Turn to proper serde_json::Value after receiving.
    result_json: String,
    time: f32
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
        BenchmarkWorker {
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) {
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        let result = self.run_benchmark(msg.benchmark);

        self.link.respond(id, result);
    }

    fn name_of_resource() -> &'static str {
        "benchmark_worker.js"
    }

}

impl BenchmarkWorker {

    fn run_benchmark(&self, benchmark: BenchmarkType) -> BenchmarkResult {
        return match benchmark {
            BenchmarkType::Dummy => run_dummy_benchmark()
        }
    }

}
