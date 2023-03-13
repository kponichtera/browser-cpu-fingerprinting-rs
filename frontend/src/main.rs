mod app;
mod profilers;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
