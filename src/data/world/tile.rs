use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub seed: usize,
    pub tile_type: TileType,
    pub tile_variant: TileVariant,
}

impl Tile {
    pub fn get_spritebuilder(&self) -> SpriteBuilder {
        match self.tile_type {
            TileType::Ground => SpriteBuilder::Ground { seed: self.seed },
            TileType::Wall { material } => SpriteBuilder::Wall {
                seed: self.seed,
                material,
            },
            TileType::ConstructedWall {
                material,
                material_shape,
                wall_feature,
            } => SpriteBuilder::ConstructedWall {
                tile_variant: self.tile_variant,
                material,
                material_shape,
                wall_feature,
            },
        }
    }
    pub fn get_symbolbuilder(&self) -> SymbolBuilder {
        match self.tile_type {
            TileType::Ground => SymbolBuilder::Ground { seed: self.seed },
            TileType::Wall { material } => SymbolBuilder::Wall {
                seed: self.seed,
                material,
            },
            TileType::ConstructedWall {
                material,
                material_shape,
                wall_feature,
            } => SymbolBuilder::ConstructedWall {
                tile_variant: self.tile_variant,
                material,
                material_shape,
                wall_feature,
            },
        }
    }
}
