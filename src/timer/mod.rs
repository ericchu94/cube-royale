use instant::Duration;
use yew::prelude::*;

use crate::app::TimerState;

#[derive(Properties, PartialEq)]
pub struct TimerProperties {
    pub state: TimerState,
    pub duration: Duration,
}

#[function_component]
pub fn Timer(props: &TimerProperties) -> Html {
    let state = props.state;
    let duration = props.duration;
    let duration = format!("{}.{:03}", duration.as_secs(), duration.subsec_millis());

    html! {
        <>
        <style>
        {".pending {
            color: green;
        }"}
        </style>
        <div class={classes!((state == TimerState::Pending).then(||"pending"))}>
            {&(*duration)}
        </div>
        </>
    }
}
