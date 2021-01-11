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
        .label("header")
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
        .label("content")
        .children(vec![
            directional(Horizontal, Stretch, Fixed(200.), 0.).build(),
            directional(Horizontal, Stretch, Stretch, 0.).build(),
        ])
        .pad_all(16.0)
        .build()
}

fn sidebar() -> Element {
    directional(Horizontal, Fixed(150.0), Stretch, 0.)
        .label("sidebar")
        .children(vec![])
        .build()
}

fn body() -> Element {
    directional(Horizontal, Stretch, Stretch, 0.)
        .label("body")
        .children(vec![content(), sidebar()])
        .build()
}

pub fn test_layout() -> Element {
    directional(Vertical, Stretch, Stretch, 0.)
        .label("layout")
        .children(vec![header(), body()])
        .build()
}
