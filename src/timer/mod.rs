use yew::prelude::*;

use crate::{app::TimerState, models::Duration};

#[derive(Properties, PartialEq)]
pub struct TimerProperties {
    pub state: TimerState,
    pub duration: Duration,
}

#[function_component]
pub fn Timer(props: &TimerProperties) -> Html {
    let state = props.state;
    let duration = if state == TimerState::Pending { Duration::default() } else { props.duration };

    html! {
        <>
        <style>
        {".pending {
            color: green;
        }"}
        </style>
        <div class={classes!((state == TimerState::Pending).then(||"pending"))}>
            {duration}
        </div>
        </>
    }
}
