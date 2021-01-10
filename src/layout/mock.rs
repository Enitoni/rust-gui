use rand::*;

use super::{
    common::{Direction::*, SizingUnit::*, *},
    dimension::Dimensions,
    directional::Directional,
    element::{Element, ElementBuilder},
    rect::Rect,
};

pub fn random_dimension(max: Int) -> Int {
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
    let space = &rng.gen_range(1..4) * 5;

    let count = {
        let available_space = match &direction {
            Horizontal => bounds.dimensions.width,
            Vertical => bounds.dimensions.height,
        };

        let max_count = (available_space as f32 / space as f32).floor() as usize;
        max_count.min(*complexity)
    };

    let mut children: Vec<Element> = Vec::with_capacity(count);

    for _ in 0..count - 1 {
        let (width, height) = direction.swap(Fixed(space), Stretch);

        let element = ElementBuilder::new()
            .directional(Directional {
                direction,
                spacing: 0,
            })
            .sizing(width, height)
            .build();

        children.push(element);
    }

    *complexity -= children.len() + 1;

    let (container_space, _) = direction.swap(bounds.dimensions.width, bounds.dimensions.width);
    let (width, height) = direction.swap(Collapse, Fixed(container_space));

    ElementBuilder::new()
        .directional(Directional {
            direction,
            spacing: 0,
        })
        .sizing(width, height)
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
