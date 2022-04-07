use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use gloo::utils::document;
use instant::{Duration, Instant};
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

use crate::players::Players;
use crate::scramble::Scramble;
use crate::timer::Timer;

#[derive(PartialEq, Clone, Copy)]
pub enum TimerState {
    Idle,
    Pending,
    Running,
    Stopped,
}

use TimerState::*;

#[function_component]
pub fn App() -> Html {
    let state = use_state(|| Idle);
    let start_time = use_state(|| Instant::now());
    let duration = use_state(|| Duration::default());

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
        let state = state.clone();
        let duration = duration.clone();
        use_effect(move || {
            let interval = Interval::new(1, move || match *state {
                Running => {
                    duration.set(start_time.elapsed());
                }
                Pending => duration.set(Duration::default()),
                _ => {}
            });

            || drop(interval)
        });
    }

    html! {
        <>
            <Scramble state={*state} />
            <Timer state={*state} duration={(*duration).clone()} />
            <Players state={*state} duration={*duration} />
        </>
    }
}
