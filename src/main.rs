mod app;
mod scramble;
mod timer;
mod players;

mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
