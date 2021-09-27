use serde::{Deserialize, Serialize};
use specs::{Component, FlaggedStorage, VecStorage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionComponent {
    pub x: i32,
    pub y: i32,
}

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
