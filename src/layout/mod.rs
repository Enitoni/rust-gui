#[cfg(test)]
mod mock;

mod common;
mod directional;

mod calculated;
mod dimension;
mod position;
mod rect;

use common::*;
use directional::*;

use self::{calculated::CalculatedElement, rect::Rect};

enum ElementKind {
    Directional(Directional),
}

pub struct Element {
    kind: ElementKind,
    children: Vec<Element>,
    dimensions: FlexibleDimensions,
}

impl Element {
    fn calculate(&self, bounds: Option<Rect>) -> CalculatedElement {
        match &self.kind {
            ElementKind::Directional(l) => l.calculate(&self, bounds),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::{
        common::{Direction::*, FlexibleDimensions, FlexibleUnit::*},
        dimension::Dimensions,
        directional::Directional,
        mock::random_directional_list,
        rect::Rect,
        Element,
        ElementKind::*,
    };

    #[test]
    fn computes_complex_directional_layout() {
        let rect = Rect::from(Dimensions::from(5000, 5000));

        let mut complexity: usize = 200;

        let list = random_directional_list(rect.clone(), &mut complexity);

        let parent = Element {
            dimensions: FlexibleDimensions {
                width: Collapse,
                height: Collapse,
            },
            kind: Directional(Directional {
                direction: Horizontal,
                spacing: 0,
            }),
            children: vec![list],
        };

        let iterations = 5000;
        let mut nodes: usize = 0;

        let time = Instant::now();

        for i in 0..iterations {
            let result = parent.calculate(Some(rect.clone()));

            if i == 0 {
                println!("{:#?}", result);
                nodes = result.nodes();
            }
        }

        let average = time.elapsed().as_secs_f32() / iterations as f32;
        let total = time.elapsed().as_secs_f32();

        println!(
            "Test took on average {} milliseconds, total execution {} milliseconds with {} iterations on {} nodes.",
            average * 1000.0,
            total * 1000.0,
            iterations,
            nodes,
        );
    }
}
