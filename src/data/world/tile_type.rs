#[derive(Clone, Copy)]
pub enum TileType {
    Ground,
    Wall,
}

impl TileType {
    pub fn collides(&self) -> bool {
        match self {
            TileType::Ground => false,
            TileType::Wall => true,
        }
    }
}

impl Default for TileType {
    fn default() -> TileType {
        unreachable!();
    }
}
