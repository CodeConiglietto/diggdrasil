use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct DrawComponent {
    pub sprite_builder: SpriteBuilder,
    //TODO: perhaps make this a seperate component
    pub symbol_builder: Option<SymbolBuilder>,
}
