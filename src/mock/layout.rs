use crate::{Direction, Directional, Element, ElementBuilder, Float, SizingUnit};
use Direction::*;
use SizingUnit::*;

use rand::{thread_rng, Rng};

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

fn server_sidebar() -> Element {
    let children = (0..7)
        .map(|_| directional(Horizontal, Fixed(40.), Fixed(40.), 0.).build())
        .collect();

    directional(Vertical, Collapse, Stretch, 16.)
        .children(children)
        .pad_all(16.)
        .build()
}

fn user_sidebar() -> Element {
    directional(Vertical, Fixed(320.0), Stretch, 24.)
        .label("sidebar")
        .pad_all(16.)
        .children(vec![user(), user(), user(), user(), user(), user(), user()])
        .build()
}

fn user() -> Element {
    let mut rng = thread_rng();

    let children = (0..2)
        .map(|_| {
            let width = &rng.gen_range(0.2..1.0);

            directional(Horizontal, Percent(*width), Fixed(16.), 0.).build()
        })
        .collect();

    directional(Horizontal, Stretch, Collapse, 16.)
        .children(vec![
            directional(Horizontal, Fixed(50.), Fixed(50.), 0.).build(),
            directional(Vertical, Stretch, Collapse, 8.)
                .children(children)
                .build(),
        ])
        .build()
}

fn body() -> Element {
    directional(Horizontal, Stretch, Stretch, 0.)
        .label("body")
        .children(vec![server_sidebar(), content(), user_sidebar()])
        .build()
}

pub fn test_layout() -> Element {
    directional(Vertical, Stretch, Stretch, 0.)
        .label("layout")
        .children(vec![header(), body()])
        .build()
}
