use specs::Entity;

use crate::prelude::*;

#[derive(Default)]
pub struct ChunkTile {
    pub tile: Tile,
    pub entities: Vec<Entity>,
}
