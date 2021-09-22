use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};

use ggez::graphics::Color;

use crate::prelude::*;

pub enum SymbolBuilder {
    Ground {
        seed: usize,
    },
    Wall {
        seed: usize,
        material: Material,
    },
    ConstructedWall {
        tile_variant: TileVariant,
        material: Material,
        material_shape: MaterialShape,
        wall_feature: Option<WallFeature>,
    },
    Humanoid {
        race: Race,
    },
    Tree,
    Log,
}

impl SymbolBuilder {
    pub fn get_symbol(&self) -> GgBunnyChar {
        match self {
            Self::Ground { seed } => GgBunnyChar {
                index: 0x2B1 + (seed) % 6,
                foreground: Color::new(0.2, 0.25, 0.2, 1.0),

                background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::Wall { material, .. } => GgBunnyChar {
                index: 0x321,
                foreground: material.get_color(), //Color::new(0.25, 0.25, 0.25, 1.0),
                background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::ConstructedWall {
                tile_variant,
                material,
                ..
            } => {
                //If variant, draw that
                //If not, draw based on tile variant
                GgBunnyChar {
                    index: tile_variant.layout.get_char_index(),
                    foreground: material.get_color(), //Color::new(0.25, 0.25, 0.25, 1.0),
                    background: None,
                    rotation: tile_variant.rotation,
                    mirror: CharMirror::None,
                }
            }
            Self::Humanoid { race } => GgBunnyChar {
                index: race.get_symbol(),
                foreground: Color::new(1.0, 0.0, 0.0, 1.0),
                background: None,
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::Tree => GgBunnyChar {
                index: 0x005,
                foreground: Color::new(0.0, 1.0, 0.0, 1.0),
                background: None,
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::Log => GgBunnyChar {
                index: 0x357,
                foreground: Color::new(0.75, 0.75, 0.0, 1.0),

                background: None,
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
        }
    }
}
