use frontend::agent::clock_worker::ClockWorker;
use yew_agent::PublicWorker;

fn main() {
    ClockWorker::register();
}
