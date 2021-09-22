use crate::prelude::*;
use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct MaterialComponent {
    pub material: Material,
    pub amount: usize,
}
