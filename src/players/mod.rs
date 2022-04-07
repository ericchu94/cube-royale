use instant::Duration;
use rand::prelude::*;
use yew::prelude::*;

#[derive(Default)]
struct Player {
    duration: Duration,
    name: String,
}

#[function_component]
pub fn Players() -> Html {
    let mut rng = thread_rng();
    let mut players = vec![];
    for i in 0..99 {
        let duration = Duration::from_millis(rng.gen_range(4000..30000));
        let name = format!("P{}", i + 1);
        players.push(Player { duration, name });
    }

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
        }
        "}
        </style>
        <div>
            {"Players"}
            <div class={"container"}>
            {players.into_iter().map(|player| {
                let d = player.duration;
                let name = player.name;
            html! {
                <div class={"player"}>
                <div>
                {name}
                </div>
                <div>
                    {format!("{}:{:03}", d.as_secs(), d.subsec_millis())}
                    </div>
                </div>
            }
        }).collect::<Html>()}
            </div>
        </div>
        </>
    }
}
