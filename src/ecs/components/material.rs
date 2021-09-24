use crate::prelude::*;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct MaterialComponent {
    pub material: Material,
    pub shape: MaterialShape,
    pub amount: usize,
}
