use yew::prelude::*;

mod scramble;

use scramble::Scramble;

mod timer;

use timer::Timer;

mod players;

use players::Players;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Scramble />
            <Timer />
            <Players />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
