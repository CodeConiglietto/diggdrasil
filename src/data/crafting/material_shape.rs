#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MaterialShape {
    Log,
    Plank,
    Brick,
    Rock,
    Stick,
}

impl MaterialShape {
    pub fn get_name(&self) -> String {
        match self {
            MaterialShape::Log => String::from("log"),
            MaterialShape::Plank => String::from("plank"),
            MaterialShape::Brick => String::from("brick"),
            MaterialShape::Rock => String::from("rock"),
            MaterialShape::Stick => String::from("stick"),
        }
    }

    pub fn get_tile_char_index(&self) -> usize {
        match self {
            MaterialShape::Log => 0x315,
            MaterialShape::Plank => 0x336,
            MaterialShape::Brick => 0x335,
            MaterialShape::Rock => 0x33E,
            MaterialShape::Stick => 0x2D7,
        }
    }

    pub fn get_item_char_index(&self) -> usize {
        match self {
            MaterialShape::Log => 0x016,
            MaterialShape::Plank => 0x357,
            MaterialShape::Brick => 0x368,
            MaterialShape::Rock => 0x2A7,
            MaterialShape::Stick => 0x222,
        }
    }
}
