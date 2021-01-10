use crate::{Dimensions, Direction, Directional, Element, ElementBuilder, Float, Rect, SizingUnit};
use Direction::*;
use SizingUnit::*;

pub fn directional(direction: Direction, width: SizingUnit, height: SizingUnit) -> ElementBuilder {
    ElementBuilder::new()
        .directional(Directional {
            direction,
            spacing: 0.0,
        })
        .sizing(width, height)
}

pub fn header() -> Element {
    directional(Horizontal, Stretch, Fixed(60.0))
        .children(vec![
            directional(Horizontal, Fixed(150.0), Stretch).build(),
            directional(Horizontal, Fixed(50.0), Stretch).build(),
            directional(Horizontal, Fixed(100.0), Stretch).build(),
            directional(Horizontal, Stretch, Stretch).build(),
            directional(Horizontal, Fixed(60.0), Stretch).build(),
        ])
        .pad_all(0.0)
        .build()
}

pub fn test_layout() -> Element {
    header()
}
