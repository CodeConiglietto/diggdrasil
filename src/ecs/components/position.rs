use serde::{Deserialize, Serialize};
use specs::{Component, FlaggedStorage, VecStorage};

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionComponent {
    pub pos: IPosition,
}

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}
