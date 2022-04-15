use std::rc::Rc;

use yew::Reducible;

use crate::models::cube_royale::{InMemoryCubeRoyale, CubeRoyale};

pub enum CubeRoyaleReducibleAction {
    NextScramble,
}

#[derive(PartialEq, Clone, Default)]
pub struct CubeRoyaleReducible(pub InMemoryCubeRoyale);

impl Reducible for CubeRoyaleReducible {
    type Action = CubeRoyaleReducibleAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();
        let cube_royale = &mut next.0;
        match action {
            NextScramble => cube_royale.next_scramble(),
        }
        next.into()
    }
}
