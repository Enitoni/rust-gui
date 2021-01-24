mod alignment;
mod calculated;
mod common;
mod constraints;
mod dimension;
mod directional;
mod element;
mod padding;
mod position;
mod rect;

pub use alignment::*;
pub use common::*;
pub use constraints::*;
pub use dimension::*;
pub use directional::*;
pub use element::*;
pub use rect::*;

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::rect::Rect;
    use crate::mock::layout::chat_box;

    #[test]
    fn computes_complex_directional_layout() {
        let rect = Rect::new(800.0, 800.0, 0.0, 0.0);
        let layout = chat_box();

        let iterations = 1;
        let mut nodes: usize = 0;

        let time = Instant::now();

        for i in 0..iterations {
            let result = layout.calculate(rect.clone(), rect.clone());

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
