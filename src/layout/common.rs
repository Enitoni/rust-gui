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

pub type Int = u32;
pub type Float = f32;

#[derive(PartialEq)]
pub enum FlexibleUnit {
    Fixed(Int),
    Collapse,
    Stretch,
}

impl FlexibleUnit {
    pub fn calculate(&self, min: Int, max: Int) -> Int {
        match self {
            FlexibleUnit::Fixed(a) => *a,
            FlexibleUnit::Stretch => max,
            FlexibleUnit::Collapse => min,
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
    pub fn calculate(&self, content: Dimensions, bounds: Dimensions) -> Dimensions {
        Dimensions {
            width: self.width.calculate(content.width, bounds.width),
            height: self.height.calculate(content.height, bounds.height),
        }
    }

    // This is used when the content is unknown, such as with a childless element
    pub fn calculate_without_content(&self, bounds: Dimensions) -> Dimensions {
        self.calculate(Dimensions::from(0, 0), bounds)
    }

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
