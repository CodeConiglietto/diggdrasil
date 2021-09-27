use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TileLayout {
    Pillar,
    Single,
    Corner,
    Straight,
    Three,
    All,
}

impl TileLayout {
    pub fn get_char_index(&self) -> usize {
        match self {
            TileLayout::Pillar => 0x189,
            TileLayout::Single => 0x32F,
            TileLayout::Corner => 0x2F4,
            TileLayout::Straight => 0x2F5,
            TileLayout::Three => 0x2F6,
            TileLayout::All => 0x2F7,
        }
    }
}

impl Default for TileLayout {
    fn default() -> TileLayout {
        TileLayout::All
    }
}
