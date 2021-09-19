use bunnyfont::{
    char_transforms::{CharMirror, CharRotation},
    ggez::GgBunnyChar,
};

use ggez::graphics::Color;

use crate::prelude::*;

pub enum SymbolBuilder {
    Wall { seed: usize },
    Ground { seed: usize },
    Humanoid { race: Race },
    Tree,
    Log,
}

impl SymbolBuilder {
    pub fn get_symbol(&self) -> GgBunnyChar {
        match self {
            Self::Wall { .. } => GgBunnyChar {
                index: 0x321,
                foreground: Color::new(0.25, 0.25, 0.25, 1.0),
                background: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
            Self::Ground { seed } => GgBunnyChar {
                index: 0x2B1 + (seed) % 6,
                foreground: Color::new(0.2, 0.25, 0.2, 1.0),

                background: Some(Color::new(0.25, 0.2, 0.2, 1.0)),
                rotation: CharRotation::None,
                mirror: CharMirror::None,
            },
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
