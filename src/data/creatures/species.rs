use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Species {
    //Sapient
    Human,
    Elf,
    Kobold,
    Goblin,
    //Other
    Deer,
}

impl Species {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Human => "human",
            Self::Elf => "elf",
            Self::Kobold => "kobold",
            Self::Goblin => "goblin",

            Self::Deer => "deer",
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::Human => Color::new(0.75, 0.0, 0.0, 1.0),
            Self::Elf => Color::new(0.0, 0.75, 0.0, 1.0),
            Self::Kobold => Color::new(0.75, 0.75, 0.75, 1.0),
            Self::Goblin => Color::new(0.75, 0.75, 0.0, 1.0),
            
            Self::Deer => Color::new(0.75, 0.75, 0.0, 1.0),
        }
    }

    pub fn get_symbol(&self) -> usize {
        let character = self.get_name().chars().next().unwrap();

        assert!(character.is_ascii_alphabetic());

        u32::from(character) as usize
    }

    pub fn get_diet(&self) -> Diet {
        match self {
            Self::Human => Diet::Omnivorous,
            Self::Elf => Diet::Herbivorous,
            Self::Kobold => Diet::Omnivorous,
            Self::Goblin => Diet::Carnivorous,

            Self::Deer => Diet::Herbivorous,
        }
    }

    pub fn get_disposition(&self) -> Disposition {
        match self {
            Self::Human => Disposition::Neutral,
            Self::Elf => Disposition::Neutral,
            Self::Kobold => Disposition::Timid,
            Self::Goblin => Disposition::Agressive,

            Self::Deer => Disposition::Timid,
        }
    }
}
