use ndarray::prelude::*;

use crate::prelude::*;

#[derive(Default)]
pub struct TileMapResource {
    pub contents: Array2<Tile>,
}
