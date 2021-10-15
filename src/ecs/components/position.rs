use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct PositionComponent {
    pub pos: IPosition,
}