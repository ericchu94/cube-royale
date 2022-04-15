use yew::prelude::*;

use crate::app::hooks::use_scramble;

#[function_component]
pub fn Scramble() -> Html {
    let scramble = use_scramble();

    html! {
        <div>
            {scramble}
        </div>
    }
}
