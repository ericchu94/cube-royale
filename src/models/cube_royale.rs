use instant::Duration;
use rand::random;

use super::scramble::Scramble;

#[derive(PartialEq, Clone, Default)]
pub struct InMemoryCubeRoyale {
    scramble: Scramble,
    history: Vec<(Scramble, Duration)>,
}

impl InMemoryCubeRoyale {
    fn next_scramble(&mut self) {
        self.scramble = random();
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

}

pub trait CubeRoyale {
    fn get_scramble(&self) -> &Scramble;

    fn complete_solve(&mut self, duration: Duration);
}
