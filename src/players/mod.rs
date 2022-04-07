use instant::Duration;
use rand::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum PlayerState {
    Alive,
    Passed,
    Failed,
    Eliminated,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Alive
    }
}

#[derive(Default, Clone, PartialEq)]
struct Player {
    name: String,
    state: PlayerState,
}

use crate::app::TimerState;

#[derive(Properties, PartialEq)]
pub struct PlayersProperties {
    pub state: TimerState,
    pub duration: Duration,
}

#[function_component]
pub fn Players(props: &PlayersProperties) -> Html {
    let state = props.state;
    let duration = props.duration;
    let players = use_state(|| {
        (0..99)
            .map(|i| {
                let name = format!("P{}", i + 1);
                Player {
                    name,
                    state: Default::default(),
                }
            })
            .collect::<Vec<Player>>()
    });
    let durations = use_state(|| [Duration::default(); 99]);

    {
        let durations = durations.clone();
        use_effect_with_deps(
            move |_| {
                if state == TimerState::Pending {
                    let mut v = (*durations).clone();
                    let mut rng = thread_rng();
                    for duration in v.iter_mut() {
                        *duration = Duration::from_millis(rng.gen_range(4000..30000));
                    }
                    durations.set(v);
                }
                || ()
            },
            state,
        );
    }

    let won = players
        .iter()
        .filter(|p| p.state == PlayerState::Passed || p.state == PlayerState::Alive)
        .count()
        == 0;
    let lost = players
        .iter()
        .filter(|p| p.state == PlayerState::Failed || p.state == PlayerState::Alive)
        .count() == 0;

    {
        let players = players.clone();
        let durations = durations.clone();
        use_effect_with_deps(
            move |_| {
                let mut v = (*players).clone();
                if (won || lost) && state == TimerState::Pending {
                    for player in v.iter_mut() {
                        player.state = PlayerState::Alive;
                    }
                } else {
                    for (i, p) in v
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, p)| p.state != PlayerState::Eliminated)
                    {
                        match state {
                            TimerState::Running => {
                                if durations[i] < duration {
                                    p.state = PlayerState::Passed;
                                }
                            }
                            TimerState::Stopped => {
                                if durations[i] > duration {
                                    p.state = PlayerState::Failed;
                                }
                            }
                            TimerState::Pending => {
                                if p.state == PlayerState::Failed {
                                    p.state = PlayerState::Eliminated;
                                } else if p.state == PlayerState::Passed {
                                    p.state = PlayerState::Alive;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                players.set(v);
                || ()
            },
            (state, duration, won),
        );
    }

    let passed = players
        .iter()
        .filter(|p| p.state == PlayerState::Passed)
        .count();
    let status = if state == TimerState::Idle && won {
        format!("Winner!")
    } else if state == TimerState::Idle && lost {
        format!("Eliminated. #{}", passed + 1)
    } else {
        format!(
            "{}/{}",
            passed,
            players
                .iter()
                .filter(|p| p.state != PlayerState::Eliminated)
                .count()
        )
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
            width: 70px;
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
                {status}
            </div>
            <div class={"container"}>
            {(*players).iter().enumerate().map(|(i, player)| {
                let d = durations[i];
                let name = &player.name;
                let mut status = format!("{:?}", player.state);
                status.make_ascii_lowercase();
            html! {
                <div class={classes!(
                    "player",
                    status,
                )}>
                <div>
                {name}
                </div>
                <div>
                    {
                        if state == TimerState::Idle || d <= duration {
                            format!("{}:{:03}", d.as_secs(), d.subsec_millis())
                        } else {
                            String::from("?")
                        }
                    }
                    </div>
                </div>
            }
        }).collect::<Html>()}
            </div>
        </div>
        </>
    }
}
