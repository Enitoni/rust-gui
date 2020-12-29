use super::{common::*, position::Position};

pub enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug)]
pub struct Rect {
    pub dimensions: Dimensions,
    pub position: Position,
}

impl Rect {
    pub fn from(width: Int, height: Int) -> Rect {
        Rect {
            dimensions: Dimensions::from(width, height),
            position: Position::from(0.0, 0.0),
        }
    }

    pub fn with_position(width: Int, height: Int, position: Position) -> Rect {
        let mut new = Rect::from(width, height);
        new.position = position;

        new
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

        Position::from(
            width / 2.0 - target_width / 2.0,
            height / 2.0 - target_height / 2.0,
        )
    }

    pub fn translate(&mut self, x: Float, y: Float) {
        self.position.translate(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use crate::layout::common::*;

    #[test]
    fn it_centers() {
        let a = Rect::from(2, 2);
        let b = Dimensions::from(1, 1);

        let result = a.center(b);

        assert_eq!(result.x, 0.5);
    }
}
