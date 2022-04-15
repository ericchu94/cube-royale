use yew::prelude::*;

use crate::models::cube_royale::CubeRoyale;
use crate::app::hooks::{use_cube_royale_context, use_scramble};

use super::app::TimerState;

#[derive(Properties, PartialEq)]
pub struct ScrambleProps {
    pub state: TimerState,
}

#[function_component]
pub fn Scramble(props: &ScrambleProps) -> Html {
    let state = props.state;
    let mut context = use_cube_royale_context();
    let scramble = use_scramble();

    {
        use_effect_with_deps(
            move |_| {
                if state == TimerState::Idle {
                    context.next_scramble();
                }
                || ()
            },
            state,
        );
    }

    html! {
        <div>
            {scramble}
        </div>
    }
}
