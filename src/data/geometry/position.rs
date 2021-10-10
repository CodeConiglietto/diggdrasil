use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub type IPosition = Position<i32>;
pub type UPosition = Position<u32>;

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn as_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: Add> Add for Position<T> {
    type Output = Position<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Position<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub for Position<T> {
    type Output = Position<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Position<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul + Clone + Copy> Mul<T> for Position<T> {
    type Output = Position<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Clone + Copy> MulAssign<T> for Position<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div + Clone + Copy> Div<T> for Position<T> {
    type Output = Position<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Clone + Copy> DivAssign<T> for Position<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Rem + Clone + Copy> Rem<T> for Position<T> {
    type Output = Position<T::Output>;

    fn rem(self, rhs: T) -> Self::Output {
        Position {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T: RemAssign + Clone + Copy> RemAssign<T> for Position<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl Sum for UPosition {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut value = Self::ZERO;

        for item in iter {
            value.x += item.x;
            value.y += item.y;
        }

        value
    }
}

impl Sum for IPosition {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut value = Self::ZERO;

        for item in iter {
            value.x += item.x;
            value.y += item.y;
        }

        value
    }
}

impl<T: Display> Display for Position<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({},{})", &self.x, &self.y)
    }
}

impl TryFrom<UPosition> for IPosition {
    type Error = <i32 as TryFrom<u32>>::Error;

    fn try_from(upos: UPosition) -> Result<Self, Self::Error> {
        Ok(Self {
            x: i32::try_from(upos.x)?,
            y: i32::try_from(upos.y)?,
        })
    }
}

impl TryFrom<IPosition> for UPosition {
    type Error = <u32 as TryFrom<i32>>::Error;

    fn try_from(upos: IPosition) -> Result<Self, Self::Error> {
        Ok(Self {
            x: u32::try_from(upos.x)?,
            y: u32::try_from(upos.y)?,
        })
    }
}

impl IPosition {
    pub fn is_adjacent(self, other: Self) -> bool {
        (self.x - other.x).abs() == 1 || (self.y - other.y).abs() == 1
    }

    pub fn is_adjacent_or_same(self, other: Self) -> bool {
        self == other || self.is_adjacent(other)
    }

    pub fn global_to_local(self) -> (IPosition, UPosition) {
        (
            Position::new(
                self.x.div_euclid(CHUNK_SIZE as i32),
                self.y.div_euclid(CHUNK_SIZE as i32),
            ),
            Position::new(
                self.x.rem_euclid(CHUNK_SIZE as i32) as u32,
                self.y.rem_euclid(CHUNK_SIZE as i32) as u32,
            ),
        )
    }

    pub fn global_from_local(chunk_pos: IPosition, local_pos: UPosition) -> Self {
        chunk_pos * CHUNK_SIZE as i32 + IPosition::try_from(local_pos).unwrap()
    }

    pub fn left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn up(self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub const ZERO: Self = Self { x: 0, y: 0 };
}

impl UPosition {
    pub fn from_idx((x, y): (usize, usize)) -> Result<Self, <u32 as TryFrom<usize>>::Error> {
        Ok(Self {
            x: u32::try_from(x)?,
            y: u32::try_from(y)?,
        })
    }

    pub fn to_idx(self) -> Result<[usize; 2], <usize as TryFrom<u32>>::Error> {
        Ok([usize::try_from(self.x)?, usize::try_from(self.y)?])
    }

    pub const ZERO: Self = Self { x: 0, y: 0 };
}
