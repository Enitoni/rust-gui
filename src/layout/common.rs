use super::dimension::Dimensions;

#[derive(Clone, Copy, Debug)]
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

    pub fn primary<T>(&self, a: T, b: T) -> T {
        self.swap(a, b).0
    }

    pub fn secondary<T>(&self, a: T, b: T) -> T {
        self.swap(a, b).1
    }

    pub fn flip(&self) -> Direction {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}

pub type Float = f32;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SizingUnit {
    Percent(Float),
    Fixed(Float),
    Collapse,
    Stretch,
}

impl SizingUnit {
    pub fn calculate(&self, content: Float, bound: Float) -> Float {
        match self {
            SizingUnit::Fixed(a) => *a,
            SizingUnit::Stretch => bound.max(0.),
            SizingUnit::Percent(p) => bound * p,
            SizingUnit::Collapse => content,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            SizingUnit::Fixed(_) => 0,
            SizingUnit::Collapse => 1,
            SizingUnit::Percent(_) => 2,
            SizingUnit::Stretch => 3,
        }
    }

    pub fn fixed(&self) -> Result<Float, &str> {
        match self {
            SizingUnit::Fixed(v) => Ok(*v),
            _ => Err("Attempt to get fixed value on dynamic unit"),
        }
    }
}

#[derive(Debug)]
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

    pub fn as_tuple(&self) -> (SizingUnit, SizingUnit) {
        (self.width, self.height)
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
