use super::{common::*, dimension::Dimensions, position::Position};

pub enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub dimensions: Dimensions,
    pub position: Position,
}

impl Rect {
    pub fn new(width: Int, height: Int, x: Float, y: Float) -> Rect {
        Rect {
            dimensions: Dimensions::new(width, height),
            position: Position::new(x, y),
        }
    }

    pub fn from_dimensions(dimensions: Dimensions) -> Rect {
        Rect {
            dimensions,
            position: Position::new(0.0, 0.0),
        }
    }

    pub fn offset(&self, side: Side, value: Int) -> Int {
        match side {
            Side::Right => self.dimensions.width - value,
            Side::Bottom => self.dimensions.height - value,
            _ => value,
        }
    }

    pub fn center(&self, dimensions: Dimensions) -> Position {
        let (width, height) = self.dimensions.to_float();
        let (target_width, target_height) = dimensions.to_float();

        Position::new(
            width / 2.0 - target_width / 2.0,
            height / 2.0 - target_height / 2.0,
        )
    }

    pub fn translate(&mut self, x: Float, y: Float) {
        self.position.translate(x, y)
    }

    /// Returns a tuple of the rect's values
    /// (width, height, x, y)
    pub fn to_tuple(&self) -> (Float, Float, Float, Float) {
        let (width, height) = self.dimensions.to_float();
        let (x, y) = self.position.to_tuple();

        (width, height, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::layout::dimension::Dimensions;

    #[test]
    fn it_centers() {
        let a = Rect::new(2, 2, 0.0, 0.0);
        let b = Dimensions::new(1, 1);

        let result = a.center(b);

        assert_eq!(result.x, 0.5);
    }
}
