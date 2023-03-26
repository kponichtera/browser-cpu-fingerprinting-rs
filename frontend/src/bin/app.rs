use frontend::gui::app_root::AppRoot;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<AppRoot>::new().render();
}
