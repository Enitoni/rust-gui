use crate::{Direction, Directional, Element, ElementBuilder, Float, SizingUnit};
use Direction::*;
use SizingUnit::*;

pub fn directional(
    direction: Direction,
    width: SizingUnit,
    height: SizingUnit,
    spacing: Float,
) -> ElementBuilder {
    ElementBuilder::new()
        .directional(Directional { direction, spacing })
        .sizing(width, height)
}

pub fn header() -> Element {
    directional(Horizontal, Stretch, Fixed(60.), 16.)
        .children(vec![
            directional(Horizontal, Fixed(50.0), Stretch, 0.).build(),
            directional(Horizontal, Fixed(50.0), Stretch, 0.).build(),
            directional(Horizontal, Fixed(50.0), Stretch, 0.).build(),
            directional(Horizontal, Stretch, Stretch, 0.).build(),
            directional(Horizontal, Fixed(80.0), Stretch, 0.).build(),
        ])
        .pad_all(16.0)
        .build()
}

fn content() -> Element {
    directional(Vertical, Stretch, Stretch, 16.)
        .children(vec![
            directional(Horizontal, Stretch, Fixed(200.), 0.).build(),
            directional(Horizontal, Stretch, Stretch, 0.).build(),
        ])
        .build()
}

fn body() -> Element {
    directional(Horizontal, Stretch, Stretch, 0.)
        .children(vec![content()])
        .build()
}

pub fn test_layout() -> Element {
    directional(Vertical, Stretch, Stretch, 0.)
        .children(vec![header(), body()])
        .build()
}
