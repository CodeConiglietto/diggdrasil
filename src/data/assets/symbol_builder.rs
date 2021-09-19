use bunnyfont::{
    char::BunnyChar,
    char_transforms::{CharMirror, CharRotation},
};

use ggez::graphics::Color;
use ndarray::prelude::*;

use crate::prelude::*;

pub enum SymbolBuilder {
    Wall { seed: usize },
    Ground { seed: usize },
    Humanoid { race: Race },
    Tree,
    Log,
}

impl SymbolBuilder {
    pub fn get_symbol(&self) -> DiggChar {
        match self {
            Self::Wall { .. } => DiggChar {
                inner: BunnyChar {
                    index: 0x321,
                    foreground: DiggColor {
                        inner: Color::new(0.25, 0.25, 0.25, 1.0),
                    },
                    background: Some(DiggColor {
                        inner: Color::new(0.0, 0.0, 0.0, 1.0),
                    }),
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Ground { seed } => DiggChar {
                inner: BunnyChar {
                    index: 0x2B1 + (seed) % 6,
                    foreground: DiggColor {
                        inner: Color::new(0.2, 0.25, 0.2, 1.0),
                    },
                    background: Some(DiggColor {
                        inner: Color::new(0.25, 0.2, 0.2, 1.0),
                    }),
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Humanoid { race } => DiggChar {
                inner: BunnyChar {
                    index: race.get_symbol(),
                    foreground: DiggColor {
                        inner: Color::new(1.0, 0.0, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Tree => DiggChar {
                inner: BunnyChar {
                    index: 0x005,
                    foreground: DiggColor {
                        inner: Color::new(0.0, 1.0, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
            Self::Log => DiggChar {
                inner: BunnyChar {
                    index: 0x357,
                    foreground: DiggColor {
                        inner: Color::new(0.75, 0.75, 0.0, 1.0),
                    },
                    background: None,
                    rotation: CharRotation::None,
                    mirror: CharMirror::None,
                },
            },
        }
    }
}
