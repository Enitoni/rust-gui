use std::fmt;

use super::common::Float;

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: Float,
    pub height: Float,
}

impl Dimensions {
    pub fn new(width: Float, height: Float) -> Dimensions {
        Dimensions { width, height }
    }

    pub fn to_tuple(&self) -> (Float, Float) {
        (self.width, self.height)
    }

    pub fn add(&mut self, width: Float, height: Float) {
        self.width += width;
        self.height += height;
    }

    pub fn subtract(&mut self, width: Float, height: Float) {
        self.width -= width;
        self.height -= height;
    }
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
