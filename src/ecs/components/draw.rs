use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct DrawComponent {
    pub seed: usize,
    pub sprite_builder: SpriteBuilder,
    //TODO: perhaps make this a seperate component
    pub symbol_builder: Option<SymbolBuilder>,
}
