use yew::prelude::*;

mod generator;

use super::app::TimerState;
use generator::generate_scramble;

#[derive(Properties, PartialEq)]
pub struct ScrambleProps {
    pub state: TimerState,
}

#[function_component]
pub fn Scramble(props: &ScrambleProps) -> Html {
    let state = props.state;

    let scramble = use_state(String::new);

    {
        let scramble = scramble.clone();
        use_effect_with_deps(
            move |_| {
                if state == TimerState::Idle {
                    scramble.set(generate_scramble());
                }
                || ()
            },
            state,
        );
    }

    html! {
        <div>
            {&(*scramble)}
        </div>
    }
}
