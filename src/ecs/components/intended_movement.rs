use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Component, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct IntendedMovementComponent {
    pub x_delta: i32,
    pub y_delta: i32,
    pub controlled: bool,
}
