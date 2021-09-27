use ggez::input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Copy, EnumIter, Serialize, Deserialize)]
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
        let (x, y) = self.get_offset();

        x.abs() == 1 && y.abs() == 1
    }

    pub fn is_orthogonal(&self) -> bool {
        let (x, y) = self.get_offset();

        x.abs() != y.abs()
    }
}
