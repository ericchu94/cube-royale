mod names;

use names::NAMES;
use rand::random;

use super::Duration;

#[derive(PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub duration: Duration,
}

pub fn generate_players() -> [Option<Player>; 99] {
    const NONE: Option<Player> = None;
    let mut players = [NONE; 99];

    for (i, p) in players.iter_mut().enumerate() {
        *p = Some(Player {
            name: NAMES[i].to_owned(),
            duration: random(),
        })
    }

    players
}
