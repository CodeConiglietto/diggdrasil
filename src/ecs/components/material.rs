use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct MaterialComponent {
    pub material: Material,
    pub shape: MaterialShape,
    pub amount: usize,
}
