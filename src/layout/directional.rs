use super::{calculated::CalculatedElement, common::*, rect::Rect, Element};

/// Represents a layout of child elements in a given direction, with a given spacing
pub struct Directional {
    /// Direction of layout
    pub direction: Direction,

    /// Spacing between children (by default 0)
    pub spacing: Int,
}

impl Directional {
    // The children need to be calculated in a specific order (fixed, collapse, stretch),
    // so this will return the new order along with the old indices
    fn sorted_child_indices(&self, children: &Vec<Element>) -> Vec<usize> {
        let mut fixed: Vec<usize> = Vec::new();
        let mut collapse: Vec<usize> = Vec::new();
        let mut stretch: Vec<usize> = Vec::new();

        for (i, child) in children.iter().enumerate() {
            let FlexibleDimensions { width, height } = &child.dimensions;
            let (unit, _) = self.direction.swap(&width, &height);

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

            let (x, y) = self.direction.swap(offset, 0.0);
            let (space, _) = self.direction.swap(width, height);

            child.rect.translate(x, y);
            offset += space as Float;
        }
    }

    fn calculate_childful(
        &self,
        element: &Element,
        parent_dimensions: Option<Dimensions>,
    ) -> CalculatedElement {
        let Element { children, .. } = element;

        let bounds = match parent_dimensions {
            Some(x) => x,
            None => {
                let dimensions = element.dimensions.fixed();

                match dimensions {
                    Ok(x) => x,
                    Err(_) => panic!("Cannot get bounds for directional element"),
                }
            }
        };

        let sorted_indices = self.sorted_child_indices(children);

        let mut accumulated_space = 0;
        let mut calculated_children: Vec<Option<CalculatedElement>> =
            Vec::with_capacity(children.len());

        // Fill the vec with nothing so we can assign later
        calculated_children.resize_with(children.len(), || None);

        for index in sorted_indices {
            let child = &children[index];

            let (unit, _) = self
                .direction
                .swap(&child.dimensions.width, &child.dimensions.height);

            let calculated_child = match unit {
                FlexibleUnit::Stretch => {
                    let (width, height) = match self.direction {
                        Direction::Horizontal => (bounds.width - accumulated_space, bounds.height),
                        Direction::Vertical => (bounds.width, bounds.height - accumulated_space),
                    };

                    child.calculate(Some(Dimensions::from(width, height)))
                }
                _ => {
                    let dimensions = Dimensions::from(
                        child.dimensions.width.calculate(bounds.width),
                        child.dimensions.height.calculate(bounds.height),
                    );

                    child.calculate(Some(dimensions))
                }
            };

            // Should replace this with a way to deal with both
            let (space, _) = self.direction.swap(
                &calculated_child.rect.dimensions.width,
                &calculated_child.rect.dimensions.height,
            );

            accumulated_space += space;
            calculated_children[index] = Some(calculated_child);
        }

        let mut calculated_children: Vec<_> = calculated_children.into_iter().flatten().collect();
        self.calculate_child_positions(&mut calculated_children);

        CalculatedElement {
            rect: Rect::from(bounds.width, bounds.height),
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
    fn calculates_childless() {
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
            children: vec![],
        };

        let result = a.calculate(Some(dimensions));

        assert_eq!(result.rect.dimensions.width, 50);
        assert_eq!(result.rect.dimensions.height, 100);
    }

    // Failing!
    #[test]
    fn calculates_collapse() {
        let dimensions = Dimensions::from(100, 100);

        let a = Element {
            kind: Directional(Directional {
                direction: Vertical,
                spacing: 0,
            }),
            dimensions: FlexibleDimensions {
                width: Stretch,
                height: Collapse,
            },
            children: vec![Element {
                kind: Directional(Directional {
                    direction: Horizontal,
                    spacing: 0,
                }),
                dimensions: FlexibleDimensions {
                    width: Fixed(50),
                    height: Fixed(50),
                },
                children: vec![],
            }],
        };

        let result = a.calculate(Some(dimensions));

        assert_eq!(result.rect.dimensions.width, 100);
        assert_eq!(result.rect.dimensions.height, 50);
    }
}
