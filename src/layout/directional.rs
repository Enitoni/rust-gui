use super::{calculated::CalculatedElement, common::*, dimension::Dimensions, rect::Rect, Element};

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

    fn calculate_childless(&self, element: &Element, bounds: Option<Rect>) -> CalculatedElement {
        let target = bounds.unwrap_or(Rect::from(Dimensions::from(0, 0)));

        let calculated = element
            .dimensions
            .calculate_without_content(target.dimensions);

        CalculatedElement::empty(calculated)
    }

    fn calculate_childful(&self, element: &Element, bounds: Option<Rect>) -> CalculatedElement {
        let Element { children, .. } = element;

        let bounds = match bounds {
            Some(x) => x.dimensions,
            None => {
                let dimensions = element.dimensions.fixed();

                match dimensions {
                    Ok(x) => x,
                    Err(_) => panic!("Cannot get bounds for directional element"),
                }
            }
        };

        let sorted_indices = self.sorted_child_indices(children);

        let mut accumulated_space = Dimensions::from(0, 0);
        let mut calculated_children: Vec<Option<CalculatedElement>> =
            Vec::with_capacity(children.len());

        // Fill the vec with nothing so we can assign later
        calculated_children.resize_with(children.len(), || None);

        // TODO: Clean this up, should calculate both flexible height and width
        for index in sorted_indices {
            let child = &children[index];

            let (unit, _) = self
                .direction
                .swap(&child.dimensions.width, &child.dimensions.height);

            let calculated_child = match unit {
                FlexibleUnit::Stretch => {
                    let (width, height) =
                        accumulated_space.diff_with_direction(self.direction, bounds);

                    child.calculate(Some(Rect::from(Dimensions::from(width, height))))
                }
                _ => {
                    let rect = Rect::from(child.dimensions.calculate_without_content(bounds));
                    child.calculate(Some(rect))
                }
            };

            accumulated_space.allocate_with_direction(
                self.direction,
                calculated_child.rect.dimensions.width,
                calculated_child.rect.dimensions.height,
            );

            calculated_children[index] = Some(calculated_child);
        }

        let mut calculated_children: Vec<_> = calculated_children.into_iter().flatten().collect();
        self.calculate_child_positions(&mut calculated_children);

        let calculated_dimensions = element.dimensions.calculate(accumulated_space, bounds);

        CalculatedElement {
            rect: Rect::from(calculated_dimensions),
            children: calculated_children,
        }
    }

    pub fn calculate(&self, element: &Element, dimensions: Option<Rect>) -> CalculatedElement {
        if element.children.len() > 0 {
            return self.calculate_childful(element, dimensions);
        }

        self.calculate_childless(element, dimensions)
    }
}

trait DirectionalDimensions {
    fn allocate_with_direction(&mut self, direction: Direction, x: Int, y: Int);

    fn diff_with_direction(&self, direction: Direction, bounds: Dimensions) -> (Int, Int);
}

impl DirectionalDimensions for Dimensions {
    fn allocate_with_direction(&mut self, direction: Direction, x: Int, y: Int) {
        let (directional, secondary) = direction.swap(&mut self.width, &mut self.height);

        *directional += x;
        *secondary = y.max(*secondary);
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
        common::FlexibleDimensions, dimension::Dimensions, rect::Rect, Direction::*, Element,
        ElementKind::*, FlexibleUnit::*,
    };

    use super::Directional;

    #[test]
    fn calculates_childless() {
        let rect = Rect::from(Dimensions::from(100, 100));

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

        let result = a.calculate(Some(rect));

        assert_eq!(result.rect.dimensions.width, 50);
        assert_eq!(result.rect.dimensions.height, 100);
    }

    #[test]
    fn calculates_collapse() {
        let rect = Rect::from(Dimensions::from(100, 100));

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

        let result = a.calculate(Some(rect));

        assert_eq!(result.rect.dimensions.width, 100);
        assert_eq!(result.rect.dimensions.height, 50);
    }
}
