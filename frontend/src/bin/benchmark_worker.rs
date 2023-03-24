use yew_agent::PrivateWorker;
use frontend::worker::BenchmarkWorker;

fn main() {
    BenchmarkWorker::register();
}
