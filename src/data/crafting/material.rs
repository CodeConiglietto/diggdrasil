use ggez::graphics::Color;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, Serialize, Deserialize)]
pub enum Material {
    Stone,
    Wood,
    Dirt,
}

impl Material {
    pub fn get_name(&self) -> String {
        match self {
            Self::Stone => String::from("rock"),
            Self::Wood => String::from("wood"),
            Self::Dirt => String::from("dirt"),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::Stone => Color::new(0.4, 0.4, 0.4, 1.0),
            Self::Wood => Color::new(0.2, 0.15, 0.0, 1.0),
            Self::Dirt => Color::new(0.3, 0.3, 0.1, 1.0),
        }
    }
}
