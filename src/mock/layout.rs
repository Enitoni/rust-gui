use crate::{AlignUnit, Direction, Directional, Element, ElementBuilder, Float, SizingUnit};
use AlignUnit::*;
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
            directional(Horizontal, Fixed(50.), Stretch, 0.).build(),
            directional(Horizontal, Fixed(300.), Stretch, 0.)
                .align(End, Start)
                .build(),
            directional(Horizontal, Fixed(70.), Stretch, 0.).build(),
            directional(Horizontal, Fixed(70.), Stretch, 0.)
                .align(Middle, Start)
                .build(),
            directional(Horizontal, Fixed(100.), Stretch, 0.)
                .align(Middle, Start)
                .build(),
            directional(Horizontal, Fixed(80.), Stretch, 0.).build(),
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
    directional(Vertical, Fixed(240.0), Stretch, 12.)
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

            directional(Horizontal, Percent(*width), Fixed(12.), 0.).build()
        })
        .collect();

    directional(Horizontal, Stretch, Collapse, 8.)
        .children(vec![
            directional(Horizontal, Fixed(35.), Fixed(35.), 0.).build(),
            directional(Vertical, Stretch, Collapse, 8.)
                .children(children)
                .build(),
        ])
        .build()
}

fn channel_list() -> Element {
    let mut rng = thread_rng();
    let amount = &rng.gen_range(5..12);

    let children = (0..*amount)
        .map(|_| directional(Horizontal, Stretch, Fixed(33.), 0.).build())
        .collect();

    directional(Vertical, Stretch, Stretch, 12.)
        .label("messages")
        .pad_all(16.)
        .children(children)
        .build()
}

fn main_sidebar() -> Element {
    directional(Vertical, Fixed(240.), Stretch, 0.)
        .children(vec![channel_list()])
        .build()
}

fn message() -> Element {
    let mut rng = thread_rng();
    let amount = &rng.gen_range(2..6);

    let children = (0..*amount)
        .map(|_| {
            let width = &rng.gen_range(0.2..1.0);

            directional(Horizontal, Percent(*width), Fixed(12.), 0.).build()
        })
        .collect();

    directional(Horizontal, Stretch, Collapse, 8.)
        .children(vec![
            directional(Horizontal, Fixed(35.), Fixed(35.), 0.).build(),
            directional(Vertical, Stretch, Collapse, 8.)
                .children(children)
                .build(),
        ])
        .build()
}

fn messages() -> Element {
    directional(Vertical, Stretch, Stretch, 12.)
        .label("messages")
        .children(vec![
            message(),
            message(),
            message(),
            message(),
            message(),
            message(),
            message(),
        ])
        .build()
}

fn content() -> Element {
    directional(Vertical, Stretch, Stretch, 16.)
        .label("content")
        .children(vec![
            messages(),
            directional(Horizontal, Stretch, Fixed(45.), 0.).build(),
        ])
        .pad_all(16.0)
        .build()
}

fn body() -> Element {
    directional(Horizontal, Stretch, Stretch, 0.)
        .label("body")
        .children(vec![
            server_sidebar(),
            main_sidebar(),
            content(),
            user_sidebar(),
        ])
        .build()
}

pub fn test_layout() -> Element {
    directional(Vertical, Stretch, Stretch, 0.)
        .label("layout")
        .children(vec![header(), body()])
        .build()
}
