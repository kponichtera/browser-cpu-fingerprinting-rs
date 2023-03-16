mod app;
mod profilers;
mod worker;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
