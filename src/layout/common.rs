use super::dimension::Dimensions;

#[derive(Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    /// Swaps a and b if self is vertical
    pub fn swap<T>(&self, a: T, b: T) -> (T, T) {
        match self {
            Direction::Horizontal => (a, b),
            Direction::Vertical => (b, a),
        }
    }

    pub fn flip(&self) -> Direction {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}

pub type Float = f32;

#[derive(PartialEq)]
pub enum SizingUnit {
    Fixed(Float),
    Collapse,
    Stretch,
}

impl SizingUnit {
    pub fn calculate(&self, min: Float, max: Float) -> Float {
        match self {
            SizingUnit::Fixed(a) => *a,
            SizingUnit::Stretch => max,
            SizingUnit::Collapse => min,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            SizingUnit::Fixed(_) => 0,
            SizingUnit::Collapse => 1,
            SizingUnit::Stretch => 2,
        }
    }

    pub fn fixed(&self) -> Result<Float, &str> {
        match self {
            SizingUnit::Fixed(v) => Ok(*v),
            _ => Err("Attempt to get fixed value on dynamic unit"),
        }
    }
}

pub struct Sizing {
    pub width: SizingUnit,
    pub height: SizingUnit,
}

impl Sizing {
    pub fn calculate(&self, content: Dimensions, bounds: Dimensions) -> Dimensions {
        Dimensions {
            width: self.width.calculate(content.width, bounds.width),
            height: self.height.calculate(content.height, bounds.height),
        }
    }

    // This is used when the content is unknown, such as with a childless element
    pub fn calculate_without_content(&self, bounds: Dimensions) -> Dimensions {
        self.calculate(Dimensions::new(0.0, 0.0), bounds)
    }

    pub fn fixed(&self) -> Result<Dimensions, &str> {
        let result: Result<Dimensions, &str> = {
            let width = self.width.fixed()?;
            let height = self.height.fixed()?;

            Ok(Dimensions::new(width, height))
        };

        match result {
            Ok(x) => Ok(x),
            Err(_) => Err("Attempt to get fixed dimensions from dynamic units"),
        }
    }
}
