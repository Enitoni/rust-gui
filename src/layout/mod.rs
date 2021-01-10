#[cfg(test)]
mod mock;

mod calculated;
mod common;
mod dimension;
mod directional;
mod element;
mod position;
mod rect;

pub use common::*;
pub use dimension::*;
pub use directional::*;

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::{
        common::{Direction::*, SizingUnit::*},
        dimension::Dimensions,
        directional::Directional,
        element::ElementBuilder,
        mock::random_directional_list,
        rect::Rect,
    };

    #[test]
    fn computes_complex_directional_layout() {
        let rect = Rect::from(Dimensions::from(5000, 5000));

        let mut complexity: usize = 200;

        let list = random_directional_list(rect.clone(), &mut complexity);

        let parent = ElementBuilder::new()
            .directional(Directional {
                direction: Horizontal,
                spacing: 0,
            })
            .sizing(Collapse, Collapse)
            .children(vec![list])
            .build();

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
