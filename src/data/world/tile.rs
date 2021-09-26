use crate::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub seed: usize,
    pub fertility: u8,
    pub tile_type: TileType,
    pub tile_variant: TileVariant,
}

impl Tile {
    pub fn get_spritebuilder(&self) -> SpriteBuilder {
        match self.tile_type {
            TileType::Ground => SpriteBuilder::Ground {fertility: self.fertility},
            TileType::Wall { material } => SpriteBuilder::Wall { material },
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
            TileType::Ground => SymbolBuilder::Ground {fertility: self.fertility},
            TileType::Wall { material } => SymbolBuilder::Wall { material },
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
