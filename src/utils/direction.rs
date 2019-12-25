use num::{Num, Signed};

use super::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    CW,
    CCW,
}

impl Direction {
    pub fn cw(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn ccw(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotated(self, rotation: Rotation) -> Direction {
        match rotation {
            Rotation::CW => self.cw(),
            Rotation::CCW => self.ccw(),
        }
    }
}

pub const DIRECTIONS: [Direction; 4] =
    [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl<T: Num + Signed + Copy> Vec2<T> {
    pub fn unit(direction: Direction) -> Vec2<T> {
        match direction {
            Direction::Up => Self::up(),
            Direction::Down => Self::down(),
            Direction::Left => Self::left(),
            Direction::Right => Self::right(),
        }
    }

    pub fn translated(self, direction: Direction) -> Vec2<T> {
        self + Self::unit(direction)
    }

    pub fn translate(&mut self, direction: Direction) {
        *self = self.translated(direction)
    }
}
