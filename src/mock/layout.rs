use crate::{AlignUnit, Direction, Element, ElementBuilder};
use AlignUnit::*;
use Direction::*;

use rand::{thread_rng, Rng};

pub fn header() -> Element {
    ElementBuilder::new()
        .directional(Horizontal, 16.)
        .sizing("Stretch", "Fixed:60")
        .label("header")
        .children(vec![
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:50", "Stretch")
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:300", "Stretch")
                .align(End, Start)
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:70", "Stretch")
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:70", "Stretch")
                .align(Middle, Start)
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:100", "Stretch")
                .align(Middle, Start)
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:80", "Stretch")
                .build(),
        ])
        .pad_all(16.0)
        .build()
}

fn server_sidebar() -> Element {
    let children = (0..7)
        .map(|_| {
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:40", "Fixed:40")
                .build()
        })
        .collect();

    ElementBuilder::new()
        .directional(Vertical, 16.)
        .sizing("Collapse", "Stretch")
        .children(children)
        .pad_all(16.)
        .build()
}

fn user_sidebar() -> Element {
    ElementBuilder::new()
        .directional(Vertical, 12.)
        .sizing("Fixed:240", "Stretch")
        .label("sidebar")
        .pad_all(16.)
        .children(vec![user(), user(), user(), user(), user(), user(), user()])
        .build()
}

fn user() -> Element {
    let mut rng = thread_rng();
    let range = &rng.gen_range(1..=2);

    let children = (0..*range)
        .map(|_| {
            let width = &rng.gen_range(0.2..1.0);
            let width = format!("Percent:{},_,_", width);

            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing(&width, "Fixed:12")
                .build()
        })
        .collect();

    ElementBuilder::new()
        .directional(Horizontal, 8.)
        .sizing("Stretch", "Collapse")
        .children(vec![
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:35", "Fixed:35")
                .build(),
            ElementBuilder::new()
                .directional(Vertical, 4.)
                .sizing("Stretch", "Collapse")
                .children(children)
                .align(Start, Middle)
                .build(),
        ])
        .build()
}

fn channel_list() -> Element {
    let mut rng = thread_rng();
    let amount = &rng.gen_range(5..12);

    let children = (0..*amount)
        .map(|_| {
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Stretch", "Fixed:33")
                .build()
        })
        .collect();

    ElementBuilder::new()
        .directional(Vertical, 12.)
        .sizing("Stretch", "Stretch")
        .label("messages")
        .pad_all(16.)
        .children(children)
        .build()
}

fn main_sidebar() -> Element {
    ElementBuilder::new()
        .directional(Vertical, 0.)
        .sizing("Fixed:240", "Stretch")
        .children(vec![channel_list()])
        .build()
}

fn message() -> Element {
    let mut rng = thread_rng();
    let amount = &rng.gen_range(2..6);

    let children = (0..*amount)
        .map(|_| {
            let width = &rng.gen_range(0.2..1.0);
            let width = format!("Percent:{},_,_", width);

            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing(&width, "Fixed:12")
                .build()
        })
        .collect();

    ElementBuilder::new()
        .directional(Horizontal, 8.)
        .sizing("Stretch", "Collapse")
        .children(vec![
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:35", "Fixed:35")
                .build(),
            ElementBuilder::new()
                .directional(Vertical, 4.)
                .sizing("Stretch", "Collapse")
                .children(children)
                .build(),
        ])
        .align(Start, End)
        .build()
}

fn messages() -> Element {
    ElementBuilder::new()
        .directional(Vertical, 12.)
        .sizing("Stretch", "Stretch")
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
    ElementBuilder::new()
        .directional(Vertical, 16.)
        .sizing("Stretch", "Stretch")
        .label("content")
        .children(vec![
            messages(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Stretch", "Fixed:45")
                .build(),
        ])
        .pad_all(16.0)
        .build()
}

fn body() -> Element {
    ElementBuilder::new()
        .directional(Horizontal, 0.)
        .sizing("Stretch", "Stretch")
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
    ElementBuilder::new()
        .directional(Vertical, 0.)
        .sizing("Stretch", "Stretch")
        .label("layout")
        .children(vec![header(), body()])
        .build()
}
