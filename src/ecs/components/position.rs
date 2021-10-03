use serde::{Deserialize, Serialize};
use specs::{Component, FlaggedStorage, VecStorage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
}

impl PositionComponent {
    pub fn get_pos_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
