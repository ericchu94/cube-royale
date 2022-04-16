use context::use_cube_royale_reducer;
use gloo::events::EventListener;
use gloo::timers::callback::Interval;
use gloo::utils::document;
use instant::{Instant};
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;

use crate::app::context::CubeRoyaleContext;
use crate::models::{CubeRoyale, Duration};
use crate::players::Players;
use crate::timer::Timer;
use crate::scramble::Scramble;

#[derive(PartialEq, Clone, Copy)]
pub enum TimerState {
    Idle,
    Pending,
    Running,
    Stopped,
}

use TimerState::*;

pub mod hooks;
pub mod context;


#[function_component]
pub fn App() -> Html {
    let cube_royale_context = use_cube_royale_reducer();

    let state = use_state(|| Idle);
    let start_time = use_state(Instant::now);
    let duration = use_state(Duration::default);

    {
        let mut cube_royale_context = cube_royale_context.clone();
        let state = state.clone();
        use_effect(move || {
            let document = document();
            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.unchecked_ref::<KeyboardEvent>();

                if e.key_code() == 32 {
                    let next_state = match *state {
                        Idle => {
                            cube_royale_context.prepare_solve();
                            Pending
                        },
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
        let duration = duration.clone();
        let mut cube_royale_context = cube_royale_context.clone();
        let state = state.clone();
        let start_time = start_time.clone();
        use_effect(move || {
            let document = document();
            let listener = EventListener::new(&document, "keyup", move |e| {
                let e = e.unchecked_ref::<KeyboardEvent>();

                if e.key_code() == 32 {
                    let next_state = match *state {
                        Stopped => {
                            cube_royale_context.complete_solve(*duration);
                            Idle
                        },
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
                    duration.set(start_time.elapsed().into());
                }
                Pending => duration.set(Duration::default()),
                _ => {}
            });

            || drop(interval)
        });
    }

    html! {
        <ContextProvider<CubeRoyaleContext> context={cube_royale_context}>
            <Scramble />
            <Timer state={*state} duration={*duration} />
            <Players state={*state} duration={*duration} />
        </ContextProvider<CubeRoyaleContext>>
    }
}
