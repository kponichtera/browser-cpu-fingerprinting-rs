mod app;

pub mod benchmarks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
