mod common;
mod directional;

mod position;
mod rect;

use common::*;
use directional::*;

enum ElementKind {
    Directional(Directional),
}

struct Element {
    kind: ElementKind,
    children: Vec<Element>,
    dimensions: FlexibleDimensions,
}

impl Element {
    fn calculate(&self, dimensions: Option<Dimensions>) -> CalculatedElement {
        match &self.kind {
            ElementKind::Directional(l) => l.calculate(&self, dimensions),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::{
        common::{Dimensions, Direction::*, FlexibleDimensions, FlexibleUnit::*},
        directional::Directional,
        Element,
        ElementKind::*,
    };

    #[test]
    fn computes_complex_directional_layout() {
        let time = Instant::now();
        let viewport = Dimensions::from(1280, 720);

        let sidebar = Element {
            kind: Directional(Directional {
                direction: Vertical,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions {
                width: Fixed(100),
                height: Stretch,
            },
            children: vec![
                Element {
                    kind: Directional(Directional {
                        direction: Horizontal,
                        spacing: 0,
                    }),
                    dimensions: FlexibleDimensions {
                        width: Stretch,
                        height: Fixed(250),
                    },
                    children: Vec::new(),
                },
                Element {
                    kind: Directional(Directional {
                        direction: Horizontal,
                        spacing: 0,
                    }),
                    dimensions: FlexibleDimensions {
                        width: Stretch,
                        height: Stretch,
                    },
                    children: Vec::new(),
                },
            ],
        };

        let content = Element {
            kind: Directional(Directional {
                direction: Horizontal,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions {
                width: Stretch,
                height: Stretch,
            },
            children: vec![
                Element {
                    kind: Directional(Directional {
                        direction: Horizontal,
                        spacing: 0,
                    }),
                    dimensions: FlexibleDimensions {
                        width: Stretch,
                        height: Fixed(200),
                    },
                    children: vec![],
                },
                Element {
                    kind: Directional(Directional {
                        direction: Horizontal,
                        spacing: 0,
                    }),
                    dimensions: FlexibleDimensions {
                        width: Stretch,
                        height: Fixed(200),
                    },
                    children: vec![],
                },
            ],
        };

        let parent = Element {
            kind: Directional(Directional {
                direction: Horizontal,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions {
                width: Stretch,
                height: Stretch,
            },
            children: vec![content, sidebar],
        };

        let iterations = 1000;

        for i in 0..iterations {
            let result = parent.calculate(Some(viewport));

            if i == 0 {
                println!("{:#?}", result);
            }
        }

        let average = time.elapsed().as_secs_f32() / iterations as f32;
        let total = time.elapsed().as_secs_f32();

        println!(
            "Test took on average {} milliseconds, total execution {} milliseconds with {} iterations.",
            average * 1000.0,
            total * 1000.0,
            iterations
        );
    }
}
