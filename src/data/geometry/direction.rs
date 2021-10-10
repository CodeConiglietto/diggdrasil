use ggez::input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use strum::EnumIter;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, EnumIter, Serialize, Deserialize, PartialEq)]
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
    pub fn from_positions(a: IPosition, b: IPosition) -> Self {
        match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
            (Ordering::Equal, Ordering::Equal) => Direction::None,
            (Ordering::Less, Ordering::Less) => Direction::UpLeft,
            (Ordering::Equal, Ordering::Less) => Direction::Up,
            (Ordering::Greater, Ordering::Less) => Direction::UpRight,
            (Ordering::Greater, Ordering::Equal) => Direction::Right,
            (Ordering::Greater, Ordering::Greater) => Direction::DownRight,
            (Ordering::Equal, Ordering::Greater) => Direction::Down,
            (Ordering::Less, Ordering::Greater) => Direction::DownLeft,
            (Ordering::Less, Ordering::Equal) => Direction::Left,
        }
    }

    pub fn from_keycode(keycode: KeyCode) -> Option<Self> {
        match keycode {
            KeyCode::Numpad1 => Some(Self::DownLeft),
            KeyCode::Numpad2 | KeyCode::Down => Some(Self::Down),
            KeyCode::Numpad3 => Some(Self::DownRight),
            KeyCode::Numpad4 | KeyCode::Left => Some(Self::Left),
            KeyCode::Numpad5 | KeyCode::Period => Some(Self::None),
            KeyCode::Numpad6 | KeyCode::Right => Some(Self::Right),
            KeyCode::Numpad7 => Some(Self::UpLeft),
            KeyCode::Numpad8 | KeyCode::Up => Some(Self::Up),
            KeyCode::Numpad9 => Some(Self::UpRight),

            _ => None,
        }
    }

    pub fn get_offset(&self) -> IPosition {
        match self {
            Direction::None => IPosition::new(0, 0),
            Direction::UpLeft => IPosition::new(-1, -1),
            Direction::Up => IPosition::new(0, -1),
            Direction::UpRight => IPosition::new(1, -1),
            Direction::Right => IPosition::new(1, 0),
            Direction::DownRight => IPosition::new(1, 1),
            Direction::Down => IPosition::new(0, 1),
            Direction::DownLeft => IPosition::new(-1, 1),
            Direction::Left => IPosition::new(-1, 0),
        }
    }

    pub fn get_angle(&self) -> Option<i32> {
        match self {
            Direction::None => None,
            Direction::Up => Some(0),
            Direction::UpRight => Some(1),
            Direction::Right => Some(2),
            Direction::DownRight => Some(3),
            Direction::Down => Some(4),
            Direction::DownLeft => Some(5),
            Direction::Left => Some(6),
            Direction::UpLeft => Some(7),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Direction::None => String::from("here"),
            Direction::UpLeft => String::from("up and left"),
            Direction::Up => String::from("up"),
            Direction::UpRight => String::from("up and right"),
            Direction::Right => String::from("right"),
            Direction::DownRight => String::from("down and right"),
            Direction::Down => String::from("down"),
            Direction::DownLeft => String::from("down and left"),
            Direction::Left => String::from("left"),
        }
    }

    pub fn is_diagonal(&self) -> bool {
        let offset = self.get_offset();

        offset.x.abs() == 1 && offset.y.abs() == 1
    }

    pub fn is_orthogonal(&self) -> bool {
        let offset = self.get_offset();

        offset.x.abs() != offset.y.abs()
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::None
    }
}
