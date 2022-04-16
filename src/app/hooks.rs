use yew::prelude::*;

use super::context::{CubeRoyaleContext};
use crate::models::{CubeRoyale, Scramble, Player};

#[hook]
pub fn use_cube_royale_context() -> CubeRoyaleContext {
    use_context::<CubeRoyaleContext>().unwrap()
}

#[hook]
pub fn use_scramble() -> Scramble {
    let context = use_cube_royale_context();
    context.get_scramble().clone()
}

#[hook]
pub fn use_players() -> [Option<Player>; 99] {
    let context = use_cube_royale_context();
    context.get_players().clone()
}
