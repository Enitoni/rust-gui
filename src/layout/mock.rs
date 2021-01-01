use rand::*;

use super::{
    common::{Direction::*, FlexibleUnit::*, *},
    dimension::Dimensions,
    directional::Directional,
    Element,
    ElementKind::*,
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

pub fn random_directional_list(bounds: Dimensions, complexity: &mut usize) -> Element {
    let mut rng = thread_rng();

    let direction = random_direction();
    let space = &rng.gen_range(1..4) * 5;

    let count = {
        let available_space = match &direction {
            Horizontal => bounds.width,
            Vertical => bounds.height,
        };

        let max_count = (available_space as f32 / space as f32).floor() as usize;
        max_count.min(*complexity)
    };

    let mut children: Vec<Element> = Vec::with_capacity(count);

    for _ in 0..count - 1 {
        let (width, height) = match &direction {
            Horizontal => (Fixed(space), Stretch),
            Vertical => (Stretch, Fixed(space)),
        };

        let element = Element {
            kind: Directional(Directional {
                direction,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions { width, height },
            children: Vec::new(),
        };

        children.push(element);
    }

    *complexity -= children.len() + 1;

    let (container_space, _) = direction.swap(bounds.width, bounds.width);
    let (width, height) = direction.swap(Fixed(container_space), Collapse);

    Element {
        kind: Directional(Directional {
            direction,
            spacing: 0,
        }),
        dimensions: FlexibleDimensions { width, height },
        children,
    }
}

pub fn random_directional(bounds: Dimensions, complexity: &mut usize) {
    let mut rng = thread_rng();
    let mut dimensions = bounds;

    let direction = random_direction();

    let child_count = rng.gen_range(1..=3.min(*complexity));
    let stretch_index = rng.gen_range(0..=child_count);

    fn child() {}
}

pub fn create_random_element() {}
