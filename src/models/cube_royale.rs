use rand::random;

use super::scramble::Scramble;

#[derive(PartialEq, Clone, Default)]
pub struct InMemoryCubeRoyale {
    pub scramble: Scramble
}

impl CubeRoyale for InMemoryCubeRoyale {
    fn next_scramble(&mut self) {
        self.scramble = random();
    }

    fn get_scramble(&self) -> &Scramble {
        &self.scramble
    }
}

pub trait CubeRoyale {
    fn next_scramble(&mut self);

    fn get_scramble(&self) -> &Scramble;
}
