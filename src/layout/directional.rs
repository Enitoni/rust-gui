use super::{common::*, rect::Rect, Element};

pub struct Directional {
    pub direction: Direction,
    pub spacing: Int,
}

impl Directional {
    fn get_directional_unit<T>(&self, a: T, b: T) -> T {
        match &self.direction {
            Direction::Horizontal => a,
            Direction::Vertical => b,
        }
    }

    fn calculate_childless(
        &self,
        element: &Element,
        dimensions: Option<Dimensions>,
    ) -> CalculatedElement {
        let FlexibleDimensions { width, height } = &element.dimensions;

        let target = dimensions.unwrap_or(Dimensions {
            width: 0,
            height: 0,
        });

        CalculatedElement::empty(
            width.calculate(target.width),
            height.calculate(target.height),
        )
    }

    fn calculate_child_positions(&self, children: &mut [CalculatedElement]) {
        let mut offset = 0.0;

        for child in children.iter_mut() {
            let Dimensions { width, height } = child.rect.dimensions;

            let (x, y) = match self.direction {
                Direction::Horizontal => (offset, 0.0),
                Direction::Vertical => (0.0, offset),
            };

            child.rect.translate(x, y);

            match self.direction {
                Direction::Horizontal => offset += width as Float,
                Direction::Vertical => offset += height as Float,
            }
        }
    }

    fn calculate_childful(
        &self,
        element: &Element,
        parent_dimensions: Option<Dimensions>,
    ) -> CalculatedElement {
        let Element { children, .. } = element;

        let dimensions = match parent_dimensions {
            Some(x) => x,
            None => {
                let dimensions = element.dimensions.fixed();

                match dimensions {
                    Ok(x) => x,
                    Err(_) => panic!("Cannot get dimensions for directional element"),
                }
            }
        };

        /*
            The children need to be calculated in a specific order (fixed, collapse, stretch),
            so this will return the new order along with the old indeices
        */
        let sorted_indices: Vec<_> = {
            let mut fixed: Vec<usize> = Vec::new();
            let mut collapse: Vec<usize> = Vec::new();
            let mut stretch: Vec<usize> = Vec::new();

            for (i, child) in children.iter().enumerate() {
                let FlexibleDimensions { width, height } = &child.dimensions;

                let unit = self.get_directional_unit(&width, &height);

                match unit {
                    FlexibleUnit::Fixed(_) => fixed.push(i),
                    FlexibleUnit::Collapse => collapse.push(i),
                    FlexibleUnit::Stretch => stretch.push(i),
                }
            }

            fixed
                .into_iter()
                .chain(collapse.into_iter())
                .chain(stretch.into_iter())
                .collect()
        };

        let mut accumulated_space = 0;
        let mut calculated_children: Vec<Option<CalculatedElement>> =
            Vec::with_capacity(children.len());

        // Fill the vec with nothing so we can assign later
        calculated_children.resize_with(children.len(), || None);

        for index in sorted_indices {
            let child = &children[index];
            let unit = self.get_directional_unit(&child.dimensions.width, &child.dimensions.height);

            let calculated_child = match unit {
                FlexibleUnit::Stretch => {
                    let (width, height) = match self.direction {
                        Direction::Horizontal => {
                            (dimensions.width - accumulated_space, dimensions.height)
                        }
                        Direction::Vertical => {
                            (dimensions.width, dimensions.height - accumulated_space)
                        }
                    };

                    child.calculate(Some(Dimensions::from(width, height)))
                }
                _ => {
                    let dimensions = Dimensions::from(
                        child.dimensions.width.calculate(dimensions.width),
                        child.dimensions.height.calculate(dimensions.height),
                    );

                    child.calculate(Some(dimensions))
                }
            };

            let space = self.get_directional_unit(
                &calculated_child.rect.dimensions.width,
                &calculated_child.rect.dimensions.height,
            );

            accumulated_space += space;
            calculated_children[index] = Some(calculated_child);
        }

        let mut calculated_children: Vec<_> = calculated_children.into_iter().flatten().collect();

        self.calculate_child_positions(&mut calculated_children);

        CalculatedElement {
            rect: Rect::from(dimensions.width, dimensions.height),
            children: calculated_children,
        }
    }

    pub fn calculate(
        &self,
        element: &Element,
        dimensions: Option<Dimensions>,
    ) -> CalculatedElement {
        if element.children.len() > 0 {
            return self.calculate_childful(element, dimensions);
        }

        self.calculate_childless(element, dimensions)
    }
}

#[cfg(test)]
mod test {
    use crate::layout::{
        common::{Dimensions, FlexibleDimensions},
        Direction::*,
        Element,
        ElementKind::*,
        FlexibleUnit::*,
    };

    use super::Directional;

    #[test]
    fn calculates_childless_correctly() {
        let dimensions = Dimensions::from(100, 100);

        let a = Element {
            kind: Directional(Directional {
                direction: Horizontal,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions {
                width: Fixed(50),
                height: Stretch,
            },
            children: Vec::new(),
        };

        let result = a.calculate(Some(dimensions));

        assert_eq!(result.rect.dimensions.width, 50);
        assert_eq!(result.rect.dimensions.height, 100);
    }
}
