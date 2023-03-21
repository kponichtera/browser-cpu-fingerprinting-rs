use frontend::clock::ClockWorker;
use yew_agent::PublicWorker;

fn main() {
    ClockWorker::register();
}
