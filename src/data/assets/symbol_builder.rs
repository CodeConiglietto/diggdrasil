use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};

use ggez::graphics::Color;

use crate::prelude::*;

pub enum SymbolBuilder {
    Ground {fertility: u8},
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
        race: Race,
    },
    Tree,
    Stick,
    Log,
    Stone,
    BerryBush,
    Berry,
    CampFire,
    Spear,
    Pick,
    Axe
}

impl SymbolBuilder {
    pub fn get_symbol(&self, seed: usize) -> Symbol {
        match self {
            Self::Ground {fertility} => {
                let grass_index = if *fertility < 8 {
                    0x000
                } else if *fertility >= 248 {
                    0x2C7
                } else {
                    0x2B0 + (*fertility as usize - 8) / 30
                };

                let (rotation, mirror) = get_random_transforms_from_seed(seed);
                
                Symbol {
                    draw_chars: vec![GgBunnyChar {
                        index: grass_index,
                        foreground: Color::new(0.2, 0.25, 0.2, 1.0),
                        background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                        rotation,
                        mirror,
                    }],
                }
            },
            Self::Wall { material, .. } => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: 0x321,
                    foreground: material.get_color(), //Color::new(0.25, 0.25, 0.25, 1.0),
                    background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::ConstructedWall {
                tile_variant,
                material,
                ..
            } => Symbol {
                draw_chars: vec![
                    GgBunnyChar {
                        index: 0x2B4,
                        foreground: Color::new(0.2, 0.25, 0.2, 1.0),

                        background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    },
                    GgBunnyChar {
                        index: tile_variant.layout.get_char_index(),
                        foreground: material.get_color(), //Color::new(0.25, 0.25, 0.25, 1.0),
                        background: None,
                        rotation: tile_variant.rotation,
                        mirror: CharMirror::None,
                    },
                ],
            },
            Self::Humanoid { race } => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: race.get_symbol(),
                    foreground: Color::new(1.0, 0.0, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::Tree => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: 0x005,
                    foreground: Color::new(0.0, 1.0, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::Stick => {
                let (rotation, mirror) = get_random_transforms_from_seed(seed);
                Symbol {
                    draw_chars: vec![GgBunnyChar {
                        index: MaterialShape::Stick.get_item_char_index(),
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation,
                        mirror,
                    }],
                }
            }
            Self::Log => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: MaterialShape::Log.get_item_char_index(),
                    foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::Stone => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: MaterialShape::Rock.get_item_char_index(),
                    foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::BerryBush => {
                let x_mirror = if (seed / 4) % 2 == 0 {
                    CharMirror::None
                } else {
                    CharMirror::MirrorX
                };
                let x_berries_mirror = if (seed / 8) % 2 == 0 {
                    CharMirror::None
                } else {
                    CharMirror::MirrorX
                };

                //TODO: make this a lazy static
                let stem_variations = [
                    GgBunnyChar {
                        index: 0x245,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: CharMirror::None,
                    },
                    GgBunnyChar {
                        index: 0x24A,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    },
                    GgBunnyChar {
                        index: 0x25B,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation180,
                        mirror: x_mirror,
                    },
                    GgBunnyChar {
                        index: 0x23A,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation180,
                        mirror: x_mirror,
                    },
                    GgBunnyChar {
                        index: 0x223,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: x_mirror,
                    },
                    GgBunnyChar {
                        index: 0x23C,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: x_mirror,
                    },
                    GgBunnyChar {
                        index: 0x24C,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: x_mirror,
                    },
                    GgBunnyChar {
                        index: 0x228,
                        foreground: Color::new(0.75, 0.75, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: x_mirror,
                    },
                    //TODO: Add a million more of these
                ];

                Symbol {
                    draw_chars: vec![
                        stem_variations[seed % stem_variations.len()],
                        GgBunnyChar {
                            index: 0x311,
                            foreground: Color::new(0.0, 0.75, 0.0, 1.0),
                            background: None,
                            rotation: CharRotation::Rotation180,
                            mirror: CharMirror::None,
                        },
                        GgBunnyChar {
                            index: 0x03A,
                            foreground: Color::new(1.0, 0.0, 0.0, 1.0),
                            background: None,
                            rotation: CharRotation::Rotation90,
                            mirror: x_berries_mirror,
                        },
                    ],
                }
            }
            Self::Berry => Symbol {
                draw_chars: vec![GgBunnyChar {
                    index: 0x189,
                    foreground: Color::new(1.0, 0.0, 0.0, 1.0),
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                }],
            },
            Self::CampFire => Symbol {
                draw_chars: vec![
                    GgBunnyChar {
                        index: 0x05E,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    },
                    GgBunnyChar {
                        index: 0x07C,
                        foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: CharMirror::None,
                    },
                ],
            },
            //TODO: find a way to pass material colours to symbols/sprites
            Self::Spear => Symbol {
                draw_chars: vec![
                    GgBunnyChar {
                        index: 0x111,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation180,
                        mirror: CharMirror::MirrorX,
                    },
                    GgBunnyChar {
                        index: 0x24F,
                        foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::None,
                    },
                ]
            },
            Self::Pick => Symbol {
                draw_chars: vec![
                    GgBunnyChar {
                        index: 0x111,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation180,
                        mirror: CharMirror::None,
                    },
                    GgBunnyChar {
                        index: 0x028,
                        foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation90,
                        mirror: CharMirror::None,
                    },
                ]
            },
            Self::Axe => Symbol {
                draw_chars: vec![
                    GgBunnyChar {
                        index: 0x15C,
                        foreground: Color::new(0.5, 0.5, 0.0, 1.0),
                        background: None,
                        rotation: CharRotation::None,
                        mirror: CharMirror::MirrorX,
                    },
                    GgBunnyChar {
                        index: 0x373,
                        foreground: Color::new(0.5, 0.5, 0.5, 1.0),
                        background: None,
                        rotation: CharRotation::Rotation270,
                        mirror: CharMirror::None,
                    },
                ]
            }
        }
    }
}
