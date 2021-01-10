use std::fmt;

use super::common::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: Float,
    pub y: Float,
}

impl Position {
    pub fn new(x: Float, y: Float) -> Position {
        Position { x, y }
    }

    pub fn relative_to(&self, position: Position) -> Position {
        Position {
            x: self.x - position.x,
            y: self.y - position.y,
        }
    }

    pub fn sum(&self, position: Position) -> Position {
        Position {
            x: self.x + position.x,
            y: self.y + position.y,
        }
    }

    pub fn translate(&mut self, x: Float, y: Float) {
        self.x = x;
        self.y = y;
    }

    pub fn to_tuple(&self) -> (Float, Float) {
        (self.x, self.y)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
