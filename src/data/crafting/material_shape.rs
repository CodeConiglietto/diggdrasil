#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MaterialShape {
    Log,
    Plank,
    Brick,
}

impl MaterialShape {
    pub fn get_name(&self) -> String {
        match self {
            MaterialShape::Log => String::from("log"),
            MaterialShape::Plank => String::from("plank"),
            MaterialShape::Brick => String::from("brick"),
        }
    }

    pub fn get_tile_char_index(&self) -> usize {
        match self {
            MaterialShape::Log => 0x315,
            MaterialShape::Plank => 0x336,
            MaterialShape::Brick => 0x335,
        }
    }
}
