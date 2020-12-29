use std::fmt;

use super::rect::Rect;

pub enum Direction {
    Horizontal,
    Vertical,
}

pub type Int = u32;
pub type Float = f32;

pub enum FlexibleUnit {
    Fixed(Int),
    Collapse,
    Stretch,
}

impl FlexibleUnit {
    pub fn calculate(&self, target: Int) -> Int {
        match self {
            FlexibleUnit::Fixed(a) => *a,
            FlexibleUnit::Stretch => target,
            FlexibleUnit::Collapse => 0,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            FlexibleUnit::Fixed(_) => 0,
            FlexibleUnit::Collapse => 1,
            FlexibleUnit::Stretch => 2,
        }
    }

    pub fn fixed(&self) -> Result<u32, &str> {
        match self {
            FlexibleUnit::Fixed(v) => Ok(*v),
            _ => Err("Attempt to get fixed value on dynamic unit"),
        }
    }
}

pub struct FlexibleDimensions {
    pub width: FlexibleUnit,
    pub height: FlexibleUnit,
}

impl FlexibleDimensions {
    pub fn fixed(&self) -> Result<Dimensions, &str> {
        let result: Result<Dimensions, &str> = {
            let width = self.width.fixed()?;
            let height = self.height.fixed()?;

            Ok(Dimensions::from(width, height))
        };

        match result {
            Ok(x) => Ok(x),
            Err(_) => Err("Attempt to get fixed dimensions from dynamic units"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: Int,
    pub height: Int,
}

impl Dimensions {
    pub fn from(width: Int, height: Int) -> Dimensions {
        Dimensions { width, height }
    }

    pub fn to_float(&self) -> (Float, Float) {
        (self.width as Float, self.height as Float)
    }
}

impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
