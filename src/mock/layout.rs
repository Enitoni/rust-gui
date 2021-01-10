use crate::{Dimensions, Direction, Directional, Element, ElementBuilder, Rect, SizingUnit};
use rand::*;
use Direction::*;
use SizingUnit::*;

pub fn random_dimension(max: u32) -> u32 {
    let max = thread_rng().gen_range(0..max);
    max - max % 5
}

pub fn random_direction() -> Direction {
    let value = thread_rng().gen_range(0..=1);

    match value {
        0 => Horizontal,
        _ => Vertical,
    }
}

pub fn random_directional_list(bounds: Rect, complexity: &mut usize) -> Element {
    let mut rng = thread_rng();

    let direction = random_direction();

    let spacing = rng.gen_range(2..4) as f32 * 8.0;
    let space = rng.gen_range(1..4) as f32 * 5.0;

    let count = {
        let available_space = match &direction {
            Horizontal => bounds.dimensions.width,
            Vertical => bounds.dimensions.height,
        };

        let max_count = (available_space / space).floor() as usize;
        max_count.min(*complexity)
    };

    let mut children: Vec<Element> = Vec::with_capacity(count);

    for _ in 0..count - 1 {
        let (width, height) = direction.swap(Fixed(space), Stretch);

        let element = ElementBuilder::new()
            .directional(Directional {
                direction,
                spacing: 0.0,
            })
            .sizing(width, height)
            .build();

        children.push(element);
    }

    *complexity -= children.len() + 1;
    let (width, height) = direction.swap(Collapse, Stretch);

    ElementBuilder::new()
        .directional(Directional { direction, spacing })
        .sizing(width, height)
        .children(children)
        .build()
}

pub fn random_directional(bounds: Dimensions, complexity: &mut usize) {
    let mut rng = thread_rng();
    let mut dimensions = bounds;

    let direction = random_direction();

    let child_count = rng.gen_range(1..=3.min(*complexity));
    let stretch_index = rng.gen_range(0..=child_count);
}

pub fn create_random_element() {}
