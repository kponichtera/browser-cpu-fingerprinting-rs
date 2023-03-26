use frontend::worker::BenchmarkWorker;
use yew_agent::PrivateWorker;

fn main() {
    console_error_panic_hook::set_once();
    BenchmarkWorker::register();
}
