use yew::prelude::*;

use crate::models::{cube_royale::CubeRoyale, scramble::Scramble};

use reducer::CubeRoyaleReducible;
use reducer::CubeRoyaleReducibleAction::*;

mod reducer;

#[derive(PartialEq, Clone)]
pub struct CubeRoyaleContext(UseReducerHandle<CubeRoyaleReducible>);

impl CubeRoyale for CubeRoyaleContext {
    fn next_scramble(&mut self) {
        self.0.dispatch(NextScramble);
    }

    fn get_scramble(&self) -> &Scramble {
        self.0.0.get_scramble()
    }
}

#[hook]
pub fn use_cube_royale_reducer() -> CubeRoyaleContext {
    CubeRoyaleContext(use_reducer(CubeRoyaleReducible::default))
}