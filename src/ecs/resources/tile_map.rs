use ndarray::prelude::*;

use crate::prelude::*;

#[derive(Default)]
pub struct TileMapResource {
    pub contents: Array2<Tile>,
}

impl TileMapResource {
    //Get neighbour offsets in (x,y) format, in (u, d, l, r) order
    pub fn get_neighbour_offsets(x: usize, y: usize) -> [(usize, usize); 4] {
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)].into()
    }

    //Get neighbour tile types in (u, d, l, r) order
    pub fn get_neighbours(&self, x: usize, y: usize) -> (TileType, TileType, TileType, TileType) {
        (
            self.contents[[x, y - 1]].tile_type,
            self.contents[[x, y + 1]].tile_type,
            self.contents[[x - 1, y]].tile_type,
            self.contents[[x + 1, y]].tile_type,
        )
    }

    pub fn change_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.contents[[x, y]] = tile;
        self.refresh_tile_variant(x, y);
    }

    pub fn refresh_tile_variant(&mut self, x: usize, y: usize) {
        self.contents[[x, y]].tile_variant =
            TileVariant::get_from_neighbours(self.get_neighbours(x, y));
    }

    pub fn refresh_tile_and_adjacent_variants(&mut self, x: usize, y: usize) {
        self.refresh_tile_variant(x, y);

        for (nx, ny) in Self::get_neighbour_offsets(x, y).iter() {
            self.refresh_tile_variant(*nx, *ny);
        }
    }

    pub fn refresh_all_tile_variants(&mut self) {
        let (width, height) = self.contents.dim();

        for x in 1..width - 1 {
            for y in 1..height - 1 {
                self.refresh_tile_variant(x, y);
            }
        }
    }
}
