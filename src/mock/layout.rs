use std::rc::Rc;

use crate::{
    parsing::parse_sizing_unit, AlignUnit, Alignment, CalculatedElement, Direction, Element,
    ElementBuilder, ElementKind, ElementLayout, Padding, Sizing,
};
use AlignUnit::*;
use Direction::*;

use rand::{thread_rng, Rng};
use rusttype::Font;

const LOREM_IPSUM: &'static str = "Suspendisse placerat lectus ultrices ipsum consectetur, et vestibulum augue luctus. Proin condimentum laoreet lacus volutpat accumsan. Aliquam commodo nulla eu elit porta, ac interdum nunc dictum. Maecenas et dui vel turpis feugiat ultrices sit amet ut augue. Vivamus enim orci, eleifend sit amet libero ut, blandit tristique orci. Donec iaculis iaculis ante, at accumsan enim congue vitae. Vivamus nec dapibus quam. Nulla lectus purus, fringilla sed eros et, hendrerit cursus sem. Integer euismod tellus turpis, at sagittis eros cursus vitae. Nulla facilisi. Ut id orci a nisl finibus tristique eu nec dui. Vestibulum vestibulum nunc quis varius blandit. Integer at erat non dolor pellentesque blandit nec vitae lorem. Praesent bibendum erat imperdiet pretium feugiat.";

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
        .label("server_sidebar")
        .build()
}

pub fn user_sidebar() -> Element {
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
        .label("user")
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

thread_local! {
    pub static FONT: Rc<Font<'static>> = Rc::new( {

        let font_data = std::fs::read("./assets/fonts/DejaVuSans.ttf").unwrap();
        rusttype::Font::try_from_vec(font_data).expect("Error constructing font")
    }
    )
}

struct TextLayoutElement {
    font: Rc<Font<'static>>,
    text: String,
    scale: rusttype::Scale,
}

impl ElementLayout for TextLayoutElement {
    fn kind_name(&self) -> String {
        "Text Element".into()
    }

    fn calculate(
        &self,
        element: &Element,
        available_bounds: crate::Rect,
        outer_bounds: crate::Rect,
    ) -> CalculatedElement {
        let mut chars = self.text.chars().map(|c| (c, self.font.glyph(c)));
        let max_width = available_bounds.dimensions.width;
        let start = available_bounds.position;
        let start = rusttype::vector(start.x, start.y);

        let mut last_breakpoint = None;
        // stored glyphs
        let mut calculated_glyphs = Vec::new();

        // last glyph on this line, for calculating kerning
        let mut last_glyph = None;
        let mut x_offset = 0.;
        let mut y_offset = 0.;
        let mut x_max: f32 = 0.;

        let v_metrics = self.font.v_metrics(self.scale);
        let line_height = (v_metrics.ascent - v_metrics.descent) + v_metrics.line_gap;

        while let Some((c, g)) = chars.next() {
            if c.is_whitespace() {
                last_breakpoint = Some((chars.clone(), calculated_glyphs.len()));
            }

            let g = g.scaled(self.scale);
            if let Some(last) = last_glyph {
                x_offset += self.font.pair_kerning(self.scale, last, g.id());
            }

            let w = g.h_metrics().advance_width;
            let next = g.positioned(start + rusttype::point(x_offset, y_offset));
            last_glyph = Some(next.id());
            x_offset += w;
            calculated_glyphs.push(next);

            if x_offset > max_width {
                x_offset = 0.;
                y_offset += line_height;
                last_glyph = None;
                if let Some((last, count)) = last_breakpoint.take() {
                    chars = last;
                    calculated_glyphs.truncate(count);
                }
            }

            x_max = x_max.max(x_offset);
        }

        CalculatedElement::from_rect(crate::Rect::new(
            x_max,
            y_offset + line_height,
            available_bounds.position.x,
            available_bounds.position.y,
        ))
    }
}

fn message() -> Element {
    let mut rng = thread_rng();
    let amount = &rng.gen_range(2..6);

    let children = (0..*amount)
        .map(|_| {
            let text_amount: usize = rng.gen_range(20..240);

            let text = FONT.with(|f| TextLayoutElement {
                font: f.clone(),
                scale: rusttype::Scale::uniform(14.),
                text: (&LOREM_IPSUM)[0..text_amount].to_owned(),
            });

            Element {
                kind: ElementKind::Extern(Box::new(text)),
                sizing: Sizing::new(
                    parse_sizing_unit("Collapse").unwrap(),
                    parse_sizing_unit("Collapse").unwrap(),
                ),
                padding: Padding::empty(),
                alignment: Alignment::new(Start, Start),
                children: Vec::new(),
                label: Some("Text".to_owned()),
            }
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
        .sizing("Stretch:800", "Stretch")
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
        .align(Middle, Start)
        .build()
}

pub fn chat_box() -> Element {
    ElementBuilder::new()
        .directional(Horizontal, 8.)
        .sizing("Stretch", "Fixed:45")
        .pad_all(16.)
        .children(vec![
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Percent:0.5,_,_", "Stretch")
                .label("percento")
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Stretch", "Stretch")
                .build(),
            ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Fixed:80", "Stretch")
                .build(),
        ])
        .build()
}

pub fn content() -> Element {
    ElementBuilder::new()
        .directional(Vertical, 16.)
        .sizing("Stretch", "Stretch")
        .label("content")
        .children(vec![messages(), chat_box()])
        .pad_all(16.0)
        .build()
}

pub fn body() -> Element {
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
