use yew::prelude::*;

use crate::models::Player;
use crate::models::{CubeRoyale, Scramble, Duration};

use reducer::CubeRoyaleReducible;
use reducer::CubeRoyaleReducibleAction::*;

mod reducer;

#[derive(PartialEq, Clone)]
pub struct CubeRoyaleContext(UseReducerHandle<CubeRoyaleReducible>);

impl CubeRoyale for CubeRoyaleContext {
    fn get_scramble(&self) -> &Scramble {
        self.0.0.get_scramble()
    }

    fn complete_solve(&mut self, duration: Duration) {
        self.0.dispatch(CompleteSolve { duration });
    }

    fn get_players(&self) -> &[Option<Player>; 99] {
        self.0.0.get_players()
    }

    fn prepare_solve(&mut self) {
        self.0.dispatch(PrepareSolve);
    }
}

#[hook]
pub fn use_cube_royale_reducer() -> CubeRoyaleContext {
    CubeRoyaleContext(use_reducer(CubeRoyaleReducible::default))
}
