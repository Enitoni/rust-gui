use super::{
    calculated::CalculatedElement, common::*, dimension::Dimensions, element::Element,
    padding::PaddedDimensions, rect::Rect,
};

#[derive(Debug)]
/// Represents a layout of child elements in a given direction, with a given spacing
pub struct Directional {
    /// Direction of layout
    pub direction: Direction,

    /// Spacing between children (by default 0)
    pub spacing: Float,
}

impl Directional {
    /// Returns a new Rect with padding and spacing accounted for
    fn occupy_bounds(&self, element: &Element, bounds: &Rect) -> Rect {
        let mut new_bounds = bounds.clone();

        new_bounds.dimensions.occupy_with_direction(
            self.direction,
            self.spacing * (element.children().len() - 1) as f32,
        );

        new_bounds.dimensions.occupy_with_padding(element.padding());

        new_bounds
    }

    // Order of calculations:
    // 1. Calculate primary, and the accumulated space, and position,
    // however secondary must also accumulate otherwise secondaries that stretch won't work
    //

    // Loops through the children to get the accumulated space needed for the final calculation
    fn calculate_accumulation(
        &self,
        sorted_indices: &Vec<usize>,
        children: &Vec<Element>,
        bounds: Rect,
    ) -> (Vec<f32>, f32) {
        let mut primary_accumulation: Vec<Float> = Vec::with_capacity(children.len());
        let mut secondary_accumulation: Float = 0.;

        let (width, height, _, _) = bounds.to_tuple();
        let available_primary = self.direction.primary(width, height);

        fn calculate_intrinsic(child: &Element, bounds: Rect) -> (Float, Float) {
            child.calculate(bounds).rect.dimensions.to_tuple()
        }

        fn calculate_stretch(
            child: &Element,
            primary_accumulation: &Vec<Float>,
            available_primary: Float,
            direction: Direction,
        ) -> (Float, Float) {
            let available = available_primary - primary_accumulation.iter().sum::<Float>();
            let (width, height) = direction.swap(available, 0.0);

            child
                .calculate(Rect::new(width, height, 0.0, 0.0))
                .rect
                .dimensions
                .to_tuple()
        }

        for index in sorted_indices {
            let child = &children[*index];
            let child_sizing = child.sizing();

            // Get the primary (directional) unit here
            let primary = self
                .direction
                .primary(&child_sizing.width, &child_sizing.height);

            let (calculated_width, calculated_height) = match primary {
                SizingUnit::Fixed(_) | SizingUnit::Collapse => {
                    calculate_intrinsic(child, bounds.clone())
                }
                SizingUnit::Stretch => calculate_stretch(
                    child,
                    &primary_accumulation,
                    available_primary,
                    self.direction,
                ),
            };

            let (primary, secondary) = self.direction.swap(calculated_width, calculated_height);

            // Push the amount of space the element took up to the primary accumulation
            primary_accumulation.push(primary);

            // Max the secondary accumulation
            secondary_accumulation = secondary_accumulation.max(secondary);
        }

        (primary_accumulation, secondary_accumulation)
    }

    fn calculate_childless(&self, element: &Element, bounds: Rect) -> CalculatedElement {
        let calculated = element
            .sizing()
            .calculate_without_content(bounds.dimensions);

        CalculatedElement::empty(calculated)
    }

    fn calculate_childful(&self, element: &Element, outer: Rect) -> CalculatedElement {
        let inner = self.occupy_bounds(element, &outer);

        let children = element.children();
        let sorted_indices = sorted_child_indices(self.direction, children);

        let (primary_accumulation, secondary_accumulation) =
            self.calculate_accumulation(&sorted_indices, children, inner);

        let mut calculated_children: Vec<Option<CalculatedElement>> = vec![None; children.len()];
        let mut primary_offset = 0.;

        let (x, y) = outer.position.to_tuple();

        for index in sorted_indices {
            let child = &children[index];
            let accumulation = &primary_accumulation[index];

            let (width, height) = self.direction.swap(accumulation, &secondary_accumulation);
            let (x_offset, y_offset) = self.direction.swap(primary_offset, 0.0);

            calculated_children[index] =
                Some(child.calculate(Rect::new(*width, *height, x_offset + x, y_offset + y)));

            primary_offset += accumulation + self.spacing;
        }

        let (width, height) = self
            .direction
            .swap(primary_accumulation.iter().sum(), secondary_accumulation);

        let calculated_bounds = element
            .sizing()
            .calculate(Dimensions::new(width, height), outer.dimensions);

        CalculatedElement {
            children: calculated_children.into_iter().flatten().collect(),
            rect: Rect::new(calculated_bounds.width, calculated_bounds.height, x, y),
        }
    }

    pub fn calculate(&self, element: &Element, dimensions: Rect) -> CalculatedElement {
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
        x: Float,
        y: Float,
    );

    fn occupy_with_direction(&mut self, direction: Direction, space_to_occupy: Float);

    fn diff_with_direction(&self, direction: Direction, bounds: Dimensions) -> (Float, Float);
}

impl DirectionalDimensions for Dimensions {
    fn allocate_with_direction(
        &mut self,
        secondary_unit: &SizingUnit,
        direction: &Direction,
        width: Float,
        height: Float,
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

    fn occupy_with_direction(&mut self, direction: Direction, space_to_occupy: Float) {
        let (directional, _) = direction.swap(&mut self.width, &mut self.height);
        *directional -= space_to_occupy;
    }

    fn diff_with_direction(&self, direction: Direction, bounds: Dimensions) -> (Float, Float) {
        let mut width = bounds.width;
        let mut height = bounds.height;

        let (_directional, _secondary) = direction.swap(&mut width, &mut height);
        let (x, y) = direction.swap(self.width, self.height);

        *_directional -= x;
        *_secondary -= y;

        (width, height)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        layout::{element::ElementBuilder, rect::Rect, Direction::*, SizingUnit::*},
        mock::layout::directional,
    };

    use super::Directional;

    #[test]
    fn calculates_childless() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let a = ElementBuilder::new()
            .directional(Directional {
                direction: Horizontal,
                spacing: 0.0,
            })
            .sizing(Fixed(50.0), Stretch)
            .build();

        let result = a.calculate(rect);

        assert_eq!(result.rect.dimensions.width, 50.0);
        assert_eq!(result.rect.dimensions.height, 100.0);
    }

    #[test]
    fn calculates_collapse() {
        let rect = Rect::new(200.0, 200.0, 0.0, 0.0);

        let child = ElementBuilder::new()
            .directional(Directional {
                direction: Horizontal,
                spacing: 0.0,
            })
            .sizing(Fixed(50.0), Fixed(50.0))
            .build();

        let a = ElementBuilder::new()
            .directional(Directional {
                direction: Vertical,
                spacing: 0.0,
            })
            .sizing(Stretch, Collapse)
            .children(vec![child])
            .build();

        let result = a.calculate(rect);

        assert_eq!(result.rect.dimensions.width, 100.0);
        assert_eq!(result.rect.dimensions.height, 50.0);
    }

    #[test]
    fn calculates_stretch() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let element = directional(Horizontal, Stretch, Collapse, 0.)
            .children(vec![
                directional(Horizontal, Fixed(20.0), Stretch, 0.).build(),
                directional(Horizontal, Fixed(20.0), Fixed(30.0), 0.).build(),
                directional(Horizontal, Stretch, Stretch, 0.).build(),
                directional(Horizontal, Fixed(20.0), Stretch, 0.).build(),
            ])
            .build();

        let result = element.calculate(rect);
        let child = &result.children[2];

        assert_eq!(result.rect.dimensions.height, 30.0);

        // 20 * 3 = 40, so remaining width is 40
        assert_eq!(child.rect.dimensions.width, 40.0);

        // Child stretches to non-directional space
        assert_eq!(child.rect.dimensions.height, 30.0);
    }

    #[test]
    fn calculates_padding() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let element = directional(Horizontal, Stretch, Fixed(50.0), 0.)
            .children(vec![directional(Horizontal, Stretch, Stretch, 0.).build()])
            .pad_all(10.0)
            .build();

        let result = element.calculate(rect);
        let child = &result.children[0];

        assert_eq!(child.rect.dimensions.height, 50.0 - (10. * 2.0));
        assert_eq!(child.rect.dimensions.width, 100.0 - (10. * 2.0));
    }
}
