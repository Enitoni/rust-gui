use super::{dimension::Dimensions, ConstraintUnit};

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
    Percent(Float, ConstraintUnit, ConstraintUnit),
    Fixed(Float),
    Collapse(ConstraintUnit),
    Stretch(ConstraintUnit),
}

impl SizingUnit {
    pub fn calculate(&self, content: Float, bound: Float) -> Float {
        match self {
            SizingUnit::Fixed(a) => *a,
            SizingUnit::Stretch(c) => {
                let max = c.calculate(bound);

                bound.min(max).max(0.)
            }
            SizingUnit::Percent(p, min, max) => {
                let min = min.calculate(bound * p);
                let max = max.calculate(bound * p);

                (bound * p).max(min).min(max)
            }
            SizingUnit::Collapse(c) => {
                let min = c.calculate(content);
                content.max(min)
            }
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            SizingUnit::Fixed(_) => 0,
            SizingUnit::Collapse(c) => 1 + c.index(),
            SizingUnit::Percent(_, _, _) => 5,
            SizingUnit::Stretch(c) => 6 + c.index(),
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
