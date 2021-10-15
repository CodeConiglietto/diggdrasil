use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

//TODO: Find a way to make this generic for multiple entity types
#[derive(Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct VegPropagationComponent {
    pub propagation_chance: u32,
    pub parent_builder: VegetationBuilder,
}