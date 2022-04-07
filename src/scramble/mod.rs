use yew::prelude::*;

mod generator;

use generator::generate_scramble;

#[function_component]
pub fn Scramble() -> Html {

    let scramble = generate_scramble();

    html! {
        <div>
            {scramble}
        </div>
    }
}
