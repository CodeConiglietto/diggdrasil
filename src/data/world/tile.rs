use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub seed: usize,
    pub tile_type: TileType,
}

impl Tile {
    pub fn get_spritebuilder(&self) -> SpriteBuilder {
        match self.tile_type {
            TileType::Wall => SpriteBuilder::Wall { seed: self.seed },
            TileType::Ground => SpriteBuilder::Ground { seed: self.seed },
        }
    }
    pub fn get_symbolbuilder(&self) -> SymbolBuilder {
        match self.tile_type {
            TileType::Wall => SymbolBuilder::Wall { seed: self.seed },
            TileType::Ground => SymbolBuilder::Ground { seed: self.seed },
        }
    }
}
