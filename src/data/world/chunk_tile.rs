use serde::{Deserialize, Serialize};
use specs::Entity;

use crate::prelude::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ChunkTile {
    pub tile: Tile,

    // TODO move this out of the serialized data to avoid re-allocating vecs for loaded chunks
    #[serde(skip)]
    pub entities: Vec<Entity>,
}
