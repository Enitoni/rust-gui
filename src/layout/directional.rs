use super::{
    calculated::CalculatedElement, common::*, dimension::Dimensions, element::Element, rect::Rect,
};

/// Represents a layout of child elements in a given direction, with a given spacing
pub struct Directional {
    /// Direction of layout
    pub direction: Direction,

    /// Spacing between children (by default 0)
    pub spacing: Int,
}

impl Directional {
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

    fn calculate_directional_child() {
        // calculate with and height here
    }

    fn calculate_childless(&self, element: &Element, bounds: Option<Rect>) -> CalculatedElement {
        let target = bounds.unwrap_or(Rect::new(0, 0, 0.0, 0.0));

        let calculated = element
            .sizing()
            .calculate_without_content(target.dimensions);

        CalculatedElement::empty(calculated)
    }

    fn calculate_childful(&self, element: &Element, bounds: Option<Rect>) -> CalculatedElement {
        let children = element.children();

        let bounds = match bounds {
            Some(x) => x.dimensions,
            None => {
                let sizing = element.sizing();
                let dimensions = sizing.fixed();

                match dimensions {
                    Ok(x) => x,
                    Err(_) => panic!("Cannot get bounds for directional element"),
                }
            }
        };

        let sorted_indices = sorted_child_indices(self.direction, children);

        let mut indices_to_correct: Vec<usize> = Vec::with_capacity(sorted_indices.len());
        let mut accumulated_space = Dimensions::new(0, 0);

        let mut calculated_children: Vec<Option<CalculatedElement>> =
            Vec::with_capacity(children.len());

        // Fill the vec with nothing so we can assign later
        calculated_children.resize_with(children.len(), || None);

        for index in sorted_indices {
            let child = &children[index];
            let child_sizing = child.sizing();

            let (unit, secondary) = self
                .direction
                .swap(&child_sizing.width, &child_sizing.height);

            let calculated_child = match unit {
                SizingUnit::Stretch => {
                    let (width, height) =
                        accumulated_space.diff_with_direction(self.direction, bounds);

                    child.calculate(Some(Rect::new(width, height, 0.0, 0.0)))
                }
                _ => {
                    let rect =
                        Rect::from_dimensions(child.sizing().calculate_without_content(bounds));
                    child.calculate(Some(rect))
                }
            };

            accumulated_space.allocate_with_direction(
                &secondary,
                &self.direction,
                calculated_child.rect.dimensions.width,
                calculated_child.rect.dimensions.height,
            );

            // The child has secondary units that
            // cannot be calculated before everything else
            if unit != secondary && *secondary == SizingUnit::Stretch {
                indices_to_correct.push(index);
            }

            calculated_children[index] = Some(calculated_child);
        }

        let calculated_dimensions = element.sizing().calculate(accumulated_space, bounds);

        // Re-calculate children with stretchy secondary sizing
        // TODO: Clean this up if possible.
        for index in indices_to_correct {
            let child = &children[index];

            let existing = match &mut calculated_children[index] {
                Some(x) => x,
                None => panic!(),
            };

            let new_calculation =
                child.calculate(Some(Rect::from_dimensions(calculated_dimensions)));

            let (_, new_secondary) = self.direction.swap(
                new_calculation.rect.dimensions.width,
                new_calculation.rect.dimensions.height,
            );

            let (_, secondary) = self.direction.swap(
                &mut existing.rect.dimensions.width,
                &mut existing.rect.dimensions.height,
            );

            *secondary = new_secondary;
        }

        let mut calculated_children: Vec<_> = calculated_children.into_iter().flatten().collect();
        self.calculate_child_positions(&mut calculated_children);

        CalculatedElement {
            rect: Rect::from_dimensions(calculated_dimensions),
            children: calculated_children,
        }
    }

    pub fn calculate(&self, element: &Element, dimensions: Option<Rect>) -> CalculatedElement {
        if element.children().len() > 0 {
            return self.calculate_childful(element, dimensions);
        }

        self.calculate_childless(element, dimensions)
    }
}

// The children need to be calculated in a specific order (fixed, collapse, stretch),
// so this will return the new order along with the old indices
fn sorted_child_indices(direction: Direction, children: &Vec<Element>) -> Vec<usize> {
    let mut fixed: Vec<usize> = Vec::new();
    let mut collapse: Vec<usize> = Vec::new();
    let mut stretch: Vec<usize> = Vec::new();

    for (i, child) in children.iter().enumerate() {
        let Sizing { width, height } = &child.sizing();
        let (directional, _) = direction.swap(&width, &height);

        match directional {
            SizingUnit::Fixed(_) => fixed.push(i),
            SizingUnit::Collapse => collapse.push(i),
            SizingUnit::Stretch => stretch.push(i),
        }
    }

    fixed
        .into_iter()
        .chain(collapse.into_iter())
        .chain(stretch.into_iter())
        .collect()
}

trait DirectionalDimensions {
    fn allocate_with_direction(
        &mut self,
        secondary: &SizingUnit,
        direction: &Direction,
        x: Int,
        y: Int,
    );

    fn diff_with_direction(&self, direction: Direction, bounds: Dimensions) -> (Int, Int);
}

impl DirectionalDimensions for Dimensions {
    fn allocate_with_direction(
        &mut self,
        secondary_unit: &SizingUnit,
        direction: &Direction,
        width: Int,
        height: Int,
    ) {
        let (directional, secondary) = direction.swap(&mut self.width, &mut self.height);
        let (x, y) = direction.swap(width, height);

        *directional += x;

        match secondary_unit {
            SizingUnit::Stretch => {}
            _ => {
                *secondary = y.max(*secondary);
            }
        }
    }

    fn diff_with_direction(&self, direction: Direction, bounds: Dimensions) -> (Int, Int) {
        let width = bounds.width;
        let height = bounds.height;

        let (mut _directional, mut _secondary) = direction.swap(width, height);
        let (x, y) = direction.swap(self.width, self.height);

        _directional -= x;
        _secondary = y;

        (width, height)
    }
}

#[cfg(test)]
mod test {
    use crate::layout::{
        dimension::Dimensions, element::ElementBuilder, rect::Rect, Direction::*, SizingUnit::*,
    };

    use super::Directional;

    #[test]
    fn calculates_childless() {
        let rect = Rect::new(100, 100, 0.0, 0.0);

        let a = ElementBuilder::new()
            .directional(Directional {
                direction: Horizontal,
                spacing: 0,
            })
            .sizing(Fixed(50), Stretch)
            .build();

        let result = a.calculate(Some(rect));

        assert_eq!(result.rect.dimensions.width, 50);
        assert_eq!(result.rect.dimensions.height, 100);
    }

    #[test]
    fn calculates_collapse() {
        let rect = Rect::new(200, 200, 0.0, 0.0);

        let child = ElementBuilder::new()
            .directional(Directional {
                direction: Horizontal,
                spacing: 0,
            })
            .sizing(Fixed(50), Fixed(50))
            .build();

        let a = ElementBuilder::new()
            .directional(Directional {
                direction: Vertical,
                spacing: 0,
            })
            .sizing(Stretch, Collapse)
            .children(vec![child])
            .build();

        let result = a.calculate(Some(rect));

        assert_eq!(result.rect.dimensions.width, 100);
        assert_eq!(result.rect.dimensions.height, 50);
    }

    #[test]
    fn calculates_stretch() {
        let rect = Rect::new(100, 100, 0.0, 0.0);

        let parent = ElementBuilder::new()
            .directional(Directional {
                direction: Vertical,
                spacing: 0,
            })
            .sizing(Collapse, Collapse)
            .children(vec![
                ElementBuilder::new()
                    .directional(Directional {
                        direction: Vertical,
                        spacing: 0,
                    })
                    .sizing(Stretch, Fixed(50))
                    .build(),
                ElementBuilder::new()
                    .directional(Directional {
                        direction: Vertical,
                        spacing: 0,
                    })
                    .sizing(Fixed(90), Fixed(50))
                    .build(),
                ElementBuilder::new()
                    .directional(Directional {
                        direction: Vertical,
                        spacing: 0,
                    })
                    .sizing(Fixed(80), Fixed(50))
                    .build(),
            ])
            .build();

        let result = parent.calculate(Some(rect));
        let child = &result.children[0];

        println!("{:?}", result);

        assert_eq!(result.rect.dimensions.width, 90);
        assert_eq!(child.rect.dimensions.width, 90);
    }
}
