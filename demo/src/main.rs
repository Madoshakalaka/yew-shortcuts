mod app;

use app::App;

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
