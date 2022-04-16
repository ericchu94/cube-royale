use yew::prelude::*;

use crate::{
    app::{hooks::use_players, TimerState},
    models::Duration,
};

#[derive(Properties, PartialEq)]
pub struct PlayersProperties {
    pub state: TimerState,
    pub duration: Duration,
}

#[function_component]
pub fn Players(props: &PlayersProperties) -> Html {
    let state = props.state;
    let duration = props.duration;
    let players = use_players();
    let round = use_state(|| 0u32);

    {
        let round = round.clone();
        use_effect_with_deps(
            move |_| {
                if state == TimerState::Pending {
                    round.set(*round + 1);
                }

                || {}
            },
            state,
        );
    }

    let total = players.iter().flatten().count();
    let won = total > 0 && players.iter().flatten().all(|p| p.duration > duration);
    let lost = total > 0 && players.iter().flatten().all(|p| p.duration < duration);

    let passed = players
        .iter()
        .flatten()
        .filter(|p| p.duration < duration)
        .count();
    let status = if state == TimerState::Idle && won {
        "Winner!".to_string()
    } else if state == TimerState::Idle && lost {
        format!("Eliminated. #{}", passed + 1)
    } else {
        format!("{}/{}", passed, total)
    };

    html! {
        <>
        <style>
        {"
        .container {
            display: flex;
            flex-wrap: wrap;
        }
        .player {
            border: 1px solid black;
            width: 80px;
            height: 40px;
            text-align: center;
            margin: 1px;
            opacity: 30%;
        }
        .failed {
            opacity: 100%;
            background: #ffaaaa;
        }
        .passed {
            opacity: 100%;
            border: 1px solid green;
            background: #aaffaa;
        }
        .eliminated {
            opacity: 0;
        }
        "}
        </style>
        <div>
            <div>
                {"Round "}{*round}{": "}{status}
            </div>
            <div class={"container"}>
            {players.iter().map(|p| {
                let status = match p {
                    None => "eliminated",
                    Some(p) => if p.duration < duration { "passed" } else if state == TimerState::Pending { "eliminated" } else { "failed" },
                };
                html! {
                    <div class={classes!(
                        "player",
                        status,
                    )}>
                    {if let Some(p) = p {
                        let d = p.duration;
                        let name = &p.name;
                        html! {
                            <>
                                <div>
                                {name}
                                </div>
                                <div>
                                    {
                                        if state == TimerState::Idle || d <= duration && state != TimerState::Pending {
                                            format!("{}", d)
                                        } else {
                                            String::from("?")
                                        }
                                    }
                                </div>
                            </>

                        }
                    } else {
                        html!{}
                    }}
                    </div>
                }
            }).collect::<Html>()}
            </div>
        </div>
        </>
    }
}
