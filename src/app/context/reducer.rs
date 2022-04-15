use std::rc::Rc;

use yew::Reducible;

use crate::models::{InMemoryCubeRoyale, CubeRoyale, Duration};
use CubeRoyaleReducibleAction::*;

pub enum CubeRoyaleReducibleAction {
    CompleteSolve { duration: Duration },
}

#[derive(PartialEq, Clone, Default)]
pub struct CubeRoyaleReducible(pub InMemoryCubeRoyale);

impl Reducible for CubeRoyaleReducible {
    type Action = CubeRoyaleReducibleAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();
        let cube_royale = &mut next.0;
        match action {
            CompleteSolve { duration } => cube_royale.complete_solve(duration),
        }
        next.into()
    }
}
