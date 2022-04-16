use rand::random;

use super::*;

#[derive(PartialEq, Clone)]
pub struct InMemoryCubeRoyale {
    scramble: Scramble,
    history: Vec<(Scramble, Duration)>,
    players: [Option<Player>; 99],
}


impl Default for InMemoryCubeRoyale {
    fn default() -> Self {
        const NONE: Option<Player> = None;
        Self {
            scramble: Scramble::default(),
            history: vec![],
            players: [NONE; 99],
        }
    }
}

impl InMemoryCubeRoyale {
    fn next_scramble(&mut self) {
        self.scramble = random();
    }

    fn next_players(&mut self) {
        if self.players.iter().all(Option::is_none) {
            self.players = generate_players();
        } else {
            for p in self.players.iter_mut().flatten() {
                p.duration = random();
            }
        }
    }

    fn eliminate_players(&mut self, duration: Duration) {
        for player in self.players.iter_mut() {
            if let Some(p) = player.as_ref() {
                if p.duration > duration {
                    player.take();
                }
            }
        }
    }
}

impl CubeRoyale for InMemoryCubeRoyale {
    fn get_scramble(&self) -> &Scramble {
        &self.scramble
    }

    fn complete_solve(&mut self, duration: Duration) {
        self.history.push((self.scramble.clone(), duration));
        self.next_scramble();
    }

    fn get_players(&self) -> &[Option<Player>; 99] {
        &self.players
    }

    fn prepare_solve(&mut self) {
        self.eliminate_players(self.history.last().map(|h| h.1).unwrap_or_default());
        self.next_players();
    }
}

pub trait CubeRoyale {
    fn get_scramble(&self) -> &Scramble;

    fn complete_solve(&mut self, duration: Duration);

    fn get_players(&self) -> &[Option<Player>; 99];

    fn prepare_solve(&mut self);
}
