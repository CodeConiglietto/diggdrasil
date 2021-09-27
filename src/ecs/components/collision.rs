use serde::{Deserialize, Serialize};
use specs::{Component, Entity, VecStorage};

use crate::prelude::*;

#[derive(Clone, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct CollisionComponent {
    #[serde(skip)]
    pub tile_collision: Option<Tile>,
    #[serde(skip)]
    pub entity_collisions: Vec<Entity>,
}
