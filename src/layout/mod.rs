mod calculated;
mod common;
mod dimension;
mod directional;
mod element;
mod padding;
mod position;
mod rect;

pub use common::*;
pub use dimension::*;
pub use directional::*;
pub use element::*;
pub use rect::*;

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::rect::Rect;
    use crate::mock::layout::test_layout;

    #[test]
    fn computes_complex_directional_layout() {
        let rect = Rect::new(500.0, 500.0, 0.0, 0.0);
        let layout = test_layout();

        let iterations = 1;
        let mut nodes: usize = 0;

        let time = Instant::now();

        for i in 0..iterations {
            let result = layout.calculate(rect.clone());

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
