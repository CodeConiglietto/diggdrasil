use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct EdibleComponent {
    pub nutrient_value: usize,
}
