use yew_agent::PrivateWorker;
use frontend::worker::BenchmarkWorker;

fn main() {
    console_error_panic_hook::set_once();
    BenchmarkWorker::register();
}
