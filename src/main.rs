mod app;
mod scramble;
mod timer;
mod players;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
