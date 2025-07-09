mod app;
mod all_icons;

use app::App;

fn main() {
    yew::set_event_bubbling(false);
    yew::Renderer::<App>::new().render();
}
