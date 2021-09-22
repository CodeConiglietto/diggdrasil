use bitflags::bitflags;

use crate::prelude::*;

bitflags! {
    #[derive(Default)]
    pub struct Directions: u16 {
        const UP_LEFT    = 0b000000001;
        const UP         = 0b000000010;
        const UP_RIGHT   = 0b000000100;
        const LEFT       = 0b000001000;
        const NONE       = 0b000010000;
        const RIGHT      = 0b000100000;
        const DOWN_LEFT  = 0b001000000;
        const DOWN       = 0b010000000;
        const DOWN_RIGHT = 0b100000000;
    }
}

impl From<Direction> for Directions {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::UpLeft => Self::UP_LEFT,
            Direction::Up => Self::UP,
            Direction::UpRight => Self::UP_RIGHT,
            Direction::Left => Self::LEFT,
            Direction::None => Self::NONE,
            Direction::Right => Self::RIGHT,
            Direction::DownLeft => Self::DOWN_LEFT,
            Direction::Down => Self::DOWN,
            Direction::DownRight => Self::DOWN_RIGHT,
        }
    }
}
