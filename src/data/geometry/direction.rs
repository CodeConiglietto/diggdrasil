#[derive(Clone, Copy)]
pub enum Direction {
    None,
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

impl Direction {
    pub fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::UpLeft => (-1, -1),
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
        }
    }
}
