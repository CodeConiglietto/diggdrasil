use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};
use ggez::graphics::Color;
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum SpriteBuilder {
    Text {
        contents: String,
    },
    Ground {
        fertility: u8,
    },
    Wall {
        material: Material,
    },
    ConstructedWall {
        tile_variant: TileVariant,
        material: Material,
        material_shape: MaterialShape,
        wall_feature: Option<WallFeature>,
    },
    Humanoid {
        species: Species,
    },
    Grass,
    Tree,
    //TODO: Make single tile sprites into something more generic
    Stick,
    Log,
    Stone,
    BerryBush,
    Berry,
    CampFire,
    Spear,
    Pick,
    Axe,
    Knife,
    Deer,
}

impl SpriteBuilder {
    pub fn get_sprite(&self, seed: usize) -> Sprite {
        match self {
            Self::Text { contents } => {
                let char_vec: Vec<_> = contents
                    .chars()
                    .map(|character| Symbol {
                        draw_chars: vec![GgBunnyChar {
                            index: u32::from(character) as usize,
                            foreground: Color::new(0.75, 0.75, 0.75, 1.0),
                            background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }],
                    })
                    .collect();

                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: Array2::from_shape_vec((1, char_vec.len()), char_vec).unwrap(),
                }
            }
            Self::Ground { fertility } => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![
                    [SymbolBuilder::Ground {
                        fertility: *fertility
                    }
                    .get_symbol(seed)],
                    [SymbolBuilder::GroundEdge{}.get_symbol(seed)]
                ],
            },
            Self::Wall { material } => {
                let mat_color = material.get_color();
                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: array![
                        [Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x321,
                                foreground: Color::BLACK, //mat_color,//Color::new(0.25, 0.25, 0.25, 1.0),
                                background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                                rotation: CharRotation::None,
                                mirror: CharMirror::None,
                            }]
                        }],
                        [Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x2B3 + (seed) % 3,
                                foreground: mat_color, //Color::new(0.4, 0.4, 0.4, 1.0),
                                background: Some(Color::new(0.25, 0.25, 0.25, 1.0),),
                                rotation: CharRotation::None,
                                mirror: CharMirror::None,
                            }]
                        }],
                        [SymbolBuilder::GroundEdge{}.get_symbol(seed)]
                    ],
                }
            }
            Self::ConstructedWall {
                tile_variant,
                material,
                material_shape,
                ..
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
                    background: if mvi == material_shape.get_tile_char_index() {
                        Some(Color::new(
                            mat_color.r * 0.5,
                            mat_color.g * 0.5,
                            mat_color.b * 0.5,
                            1.0,
                        ))
                    } else {
                        None
                    },
                    rotation: mvr,
                    mirror: mvm,
                };

                Sprite {
                    origin_x: 0,
                    origin_y: 1,
                    contents: array![
                        [Symbol {
                            draw_chars: vec![top_variant_fill, top_variant_silhouette]
                        }],
                        [Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x2B4,
                                    foreground: Color::new(0.2, 0.25, 0.2, 1.0),
                                    background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                                    rotation: CharRotation::None,
                                    mirror: CharMirror::None,
                                },
                                mid_variant_char
                            ]
                        }],
                        [SymbolBuilder::GroundEdge{}.get_symbol(seed)]
                    ],
                }
            }
            Self::Humanoid { species } => Sprite {
                origin_x: 0,
                origin_y: 1,
                contents: array![
                    [Symbol {
                        draw_chars: vec![GgBunnyChar {
                            index: 0x00C,
                            foreground: species.get_color(),
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]
                    }],
                    [Symbol {
                        draw_chars: vec![GgBunnyChar {
                            index: 0x05E,
                            foreground: species.get_color(),
                            background: None,
                            rotation: CharRotation::None,
                            mirror: CharMirror::None,
                        }]
                    }]
                ],
            },
            Self::Grass => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Grass.get_symbol(seed)]],
            },
            Self::Tree => Sprite {
                origin_x: 2,
                origin_y: 4,
                contents: array![
                    //0
                    [
                        Symbol::empty(),
                        Symbol::empty(),
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x32F,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::Rotation270,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x310,
                                foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::Rotation90,
                                mirror: Mirror::None
                            }]
                        },
                        Symbol::empty()
                    ],
                    //1
                    [
                        Symbol::empty(),
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x312,
                                foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::None,
                                mirror: Mirror::None
                            }]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x2F7,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::Rotation270,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x32F,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::Rotation270,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol::empty()
                    ],
                    //2
                    [
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x310,
                                foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::None,
                                mirror: Mirror::None
                            }]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x32F,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::Rotation270,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x31A,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x2FB,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x32F,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x2B3,
                                    foreground: Color::new(0.0, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                    ],
                    //3
                    [
                        Symbol::empty(),
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x2F4,
                                foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::Rotation270,
                                mirror: Mirror::None
                            }]
                        },
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x31B,
                                foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::Rotation270,
                                mirror: Mirror::None
                            },]
                        },
                        Symbol {
                            draw_chars: vec![GgBunnyChar {
                                index: 0x318,
                                foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                background: None,
                                rotation: Rotation::Rotation180,
                                mirror: Mirror::None
                            }]
                        },
                        Symbol::empty()
                    ],
                    //4
                    [
                        Symbol::empty(),
                        Symbol::empty(),
                        Symbol {
                            draw_chars: vec![
                                GgBunnyChar {
                                    index: 0x2DD,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::None,
                                    mirror: Mirror::None
                                },
                                GgBunnyChar {
                                    index: 0x319,
                                    foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                                    background: None,
                                    rotation: Rotation::Rotation90,
                                    mirror: Mirror::None
                                }
                            ]
                        },
                        Symbol::empty(),
                        Symbol::empty()
                    ],
                ],
            },
            Self::Stick => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Stick.get_symbol(seed)]],
            },
            Self::Log => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Log.get_symbol(seed)]],
            },
            Self::Stone => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Stone.get_symbol(seed)]],
            },
            Self::BerryBush => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::BerryBush.get_symbol(seed)]],
            },
            Self::Berry => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Berry.get_symbol(seed)]],
            },
            Self::CampFire => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::CampFire.get_symbol(seed)]],
            },
            Self::Spear => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Spear.get_symbol(seed)]],
            },
            Self::Pick => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Pick.get_symbol(seed)]],
            },
            Self::Axe => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Axe.get_symbol(seed)]],
            },
            Self::Knife => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Knife.get_symbol(seed)]],
            },
            Self::Deer => Sprite {
                origin_x: 0,
                origin_y: 0,
                contents: array![[SymbolBuilder::Deer.get_symbol(seed)]],
            },
        }
    }
}
