use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct FieldOfViewComponent {
    pub shadowcast: Shadowcast,
}

impl FieldOfViewComponent {
    pub fn new(radius: usize) -> Self {
        Self {
            shadowcast: Shadowcast::new(radius),
        }
    }
}
