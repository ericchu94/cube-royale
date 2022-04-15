use yew::prelude::*;

use super::context::{CubeRoyaleContext};
use crate::models::scramble::Scramble;
use crate::models::cube_royale::CubeRoyale;

#[hook]
pub fn use_cube_royale_context() -> CubeRoyaleContext {
    use_context::<CubeRoyaleContext>().unwrap()
}

#[hook]
pub fn use_scramble() -> Scramble {
    let context = use_cube_royale_context();
    context.get_scramble().clone()
}
