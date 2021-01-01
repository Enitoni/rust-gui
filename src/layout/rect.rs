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
    pub fn from(dimensions: Dimensions) -> Rect {
        Rect {
            dimensions,
            position: Position::from(0.0, 0.0),
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
    use crate::layout::dimension::Dimensions;

    #[test]
    fn it_centers() {
        let a = Rect::from(Dimensions::from(2, 2));
        let b = Dimensions::from(1, 1);

        let result = a.center(b);

        assert_eq!(result.x, 0.5);
    }
}
