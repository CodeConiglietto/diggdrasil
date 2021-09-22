use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};

use ggez::graphics::Color;
use ndarray::prelude::*;

use crate::prelude::*;

pub enum SpriteBuilder {
    Text {
        contents: String,
    },
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

impl SpriteBuilder {
    pub fn get_sprite(&self) -> Sprite {
        match self {
            Self::Text { contents } => {
                let char_vec: Vec<_> = contents
                    .chars()
                    .map(|character| {
                        vec![GgBunnyChar {
                            index: u32::from(character) as usize,
                            foreground: Color::new(0.75, 0.75, 0.75, 1.0),
                            background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]
                    })
                    .collect();

                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: Array2::from_shape_vec((1, char_vec.len()), char_vec).unwrap(),
                }
            }
            Self::Wall { seed, material } => {
                let mat_color = material.get_color();
                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: array![
                        [vec![GgBunnyChar {
                            index: 0x321,
                            foreground: Color::BLACK, //mat_color,//Color::new(0.25, 0.25, 0.25, 1.0),
                            background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]],
                        [vec![GgBunnyChar {
                            index: 0x2B3 + (seed) % 3,
                            foreground: mat_color, //Color::new(0.4, 0.4, 0.4, 1.0),
                            background: Some(Color::new(0.25, 0.25, 0.25, 1.0),),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]],
                        [vec![GgBunnyChar {
                            index: 0x2B0 + (seed) % 8,
                            foreground: Color::new(0.2, 0.15, 0.15, 1.0),
                            background: Some(Color::new(0.15, 0.1, 0.1, 1.0)),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]]
                    ],
                }
            }
            Self::ConstructedWall {
                tile_variant,
                material,
                material_shape,
                wall_feature,
            } => {
                let mat_color = material.get_color();

                //get top based on tile variant

                //If there is a wall feature:
                //-get mid based on wall feature
                //else
                //-get mid based on material shape

                let top_variant_silhouette = GgBunnyChar {
                    index: tile_variant.layout.get_char_index(),
                    foreground: Color::BLACK,
                    background: None,
                    rotation: tile_variant.rotation,
                    mirror: CharMirror::None,
                };

                let (tvi, tvr, tvm) = tile_variant.get_top_fill_char();
                let top_variant_fill = GgBunnyChar {
                    index: tvi,
                    foreground: material.get_color(),
                    background: None,
                    rotation: tvr,
                    mirror: tvm,
                };

                let (mvi, mvr, mvm) = tile_variant.get_mid_char((
                    material_shape.get_tile_char_index(),
                    Rotation::Rotation180,
                    Mirror::None,
                ));
                let mid_variant_char = GgBunnyChar {
                    index: mvi,
                    foreground: mat_color,
                    background: Some(Color::new(0.25, 0.25, 0.25, 1.0)),
                    rotation: mvr,
                    mirror: mvm,
                };

                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: array![
                        [vec![top_variant_fill, top_variant_silhouette]],
                        [vec![mid_variant_char]],
                        [vec![GgBunnyChar {
                            index: 0x2B0,
                            foreground: Color::new(0.2, 0.15, 0.15, 1.0),
                            background: Some(Color::new(0.15, 0.1, 0.1, 1.0)),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]]
                    ],
                }
            }
            Self::Ground { seed } => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![
                    [vec![GgBunnyChar {
                        index: 0x2B1 + (seed) % 6,
                        foreground: Color::new(0.2, 0.25, 0.2, 1.0),
                        background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]],
                    [vec![GgBunnyChar {
                        index: 0x2B4 + (seed) % 4,
                        foreground: Color::new(0.2, 0.15, 0.15, 1.0),
                        background: Some(Color::new(0.15, 0.1, 0.1, 1.0)),
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]]
                ],
            },
            Self::Humanoid { race } => Sprite {
                origin_x: 0,
                origin_y: 1,
                contents: array![
                    [vec![GgBunnyChar {
                        index: 0x00C,
                        foreground: race.get_color(),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]],
                    [vec![GgBunnyChar {
                        index: 0x05E,
                        foreground: race.get_color(),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]]
                ],
            },
            Self::Tree => Sprite {
                origin_x: 0,
                origin_y: 3,
                contents: array![
                    [vec![GgBunnyChar {
                        index: 0x02A,
                        foreground: Color::new(0.0, 1.0, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]],
                    [vec![GgBunnyChar {
                        index: 0x07C,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]],
                    [vec![GgBunnyChar {
                        index: 0x07C,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]],
                    [vec![GgBunnyChar {
                        index: 0x07C,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    }]]
                ],
            },
            Self::Log => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[vec![GgBunnyChar {
                    index: 0x357,
                    foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }]],],
            },
        }
    }
}
