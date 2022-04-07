use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use gloo::utils::document;
use instant::Instant;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(PartialEq)]
enum TimerState {
    Idle,
    Pending,
    Running,
    Stopped,
}

use TimerState::*;

#[function_component]
pub fn Timer() -> Html {
    let state = use_state(|| Idle);
    let start_time = use_state(|| Instant::now());
    let duration = use_state(|| String::from("0.000"));

    {
        let state = state.clone();
        use_effect(move || {
            let document = document();
            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.unchecked_ref::<KeyboardEvent>();

                if e.key_code() == 32 {
                    let next_state = match *state {
                        Stopped => Pending,
                        Running => Stopped,
                        _ => Pending,
                    };
                    state.set(next_state);
                }
            });

            || drop(listener)
        });
    }

    {
        let state = state.clone();
        let start_time = start_time.clone();
        use_effect(move || {
            let document = document();
            let listener = EventListener::new(&document, "keyup", move |e| {
                let e = e.unchecked_ref::<KeyboardEvent>();

                if e.key_code() == 32 {
                    let next_state = match *state {
                        Stopped => Idle,
                        Pending => Running,
                        _ => Idle,
                    };
                    state.set(next_state);
                    start_time.set(Instant::now())
                }
            });

            || drop(listener)
        });
    }

    {
        let duration = duration.clone();
        let state = state.clone();
        use_effect(move || {
            let interval = Interval::new(1, move || match *state {
                Running => {
                    let d = start_time.elapsed();
                    duration.set(format!("{}.{:03}", d.as_secs(), d.subsec_millis()));
                }
                Pending => duration.set(String::from("0.000")),
                _ => {}
            });

            || drop(interval)
        });
    }

    html! {
        <>
        <style>
        {".pending {
            color: green;
        }"}
        </style>
        <div class={classes!((*state == Pending).then(||"pending"))}>
            {&(*duration)}
        </div>
        </>
    }
}
