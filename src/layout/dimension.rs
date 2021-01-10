use std::fmt;

use super::common::{Float, Int};

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: Int,
    pub height: Int,
}

impl Dimensions {
    pub fn new(width: Int, height: Int) -> Dimensions {
        Dimensions { width, height }
    }

    pub fn to_float(&self) -> (Float, Float) {
        (self.width as Float, self.height as Float)
    }

    pub fn to_tuple(&self) -> (Int, Int) {
        (self.width, self.height)
    }

    pub fn add(&mut self, width: Int, height: Int) {
        self.width += width;
        self.height += height;
    }
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
