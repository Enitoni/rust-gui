use crate::Dimensions;

use super::common::*;
#[derive(Debug)]
pub struct Padding {
    top: Float,
    left: Float,
    right: Float,
    bottom: Float,
}

impl Padding {
    pub fn new(top: Float, bottom: Float, left: Float, right: Float) -> Padding {
        Padding {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn empty() -> Padding {
        Padding::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn to_tuple(&self) -> (Float, Float, Float, Float) {
        (self.top, self.bottom, self.left, self.right)
    }
}

pub trait PaddedDimensions {
    fn occupy_with_padding(&mut self, padding: &Padding);
}

impl PaddedDimensions for Dimensions {
    fn occupy_with_padding(&mut self, padding: &Padding) {
        let x = padding.left + padding.right;
        let y = padding.top + padding.bottom;

        self.subtract(x, y);
    }
}
