use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Debug, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct VelocityComponent {
    pub x: i32,
    pub y: i32,
}
