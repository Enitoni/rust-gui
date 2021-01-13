use super::{
    alignment::*, calculated::CalculatedElement, common::*, dimension::Dimensions,
    element::Element, padding::PaddedDimensions, rect::Rect,
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
    // Returns a new Rect with padding and spacing accounted for
    fn occupy_bounds(&self, element: &Element, bounds: &Rect) -> Rect {
        let mut new_bounds = bounds.clone();

        new_bounds.dimensions.occupy_with_padding(element.padding());
        new_bounds
    }

    // Loops through the children to get the accumulated space needed for the final calculation
    fn calculate_accumulation(
        &self,
        sorted_indices: &Vec<usize>,
        children: &Vec<Element>,
        bounds: Rect,
    ) -> (Vec<f32>, Vec<f32>) {
        let mut primary_accumulation: Vec<Float> = vec![0.; children.len()];
        let mut secondary_accumulation: Vec<Float> = vec![0.; children.len()];

        let (width, height, _, _) = bounds.to_tuple();
        let available_primary = self.direction.primary(width, height) - self.spacing;

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
            let (primary_unit, secondary_unit) = self
                .direction
                .swap(&child_sizing.width, &child_sizing.height);

            let (calculated_width, calculated_height) = match primary_unit {
                SizingUnit::Fixed(_) | SizingUnit::Collapse => {
                    calculate_intrinsic(child, bounds.clone())
                }
                SizingUnit::Stretch | SizingUnit::Percent(_) => calculate_stretch(
                    child,
                    &primary_accumulation,
                    available_primary,
                    self.direction,
                ),
            };

            let (primary, secondary) = self.direction.swap(calculated_width, calculated_height);

            primary_accumulation[*index] = primary;
            secondary_accumulation[*index] = secondary;
        }

        (primary_accumulation, secondary_accumulation)
    }

    fn calculate_position_offsets(
        &self,
        accumulations: &Vec<Float>,
        children: &Vec<Element>,
        bounds: Dimensions,
    ) -> (f32, f32) {
        let (width, height) = bounds.to_tuple();
        let primary = self.direction.primary(width, height);

        let mut start_accumulation = -self.spacing;
        let mut middle_accumulation = -self.spacing;
        let mut end_accumulation = -self.spacing;

        for i in 0..children.len() {
            let child = &children[i];
            let accumulation = *&accumulations[i];

            let (vertical, horizontal) = child.alignment().as_tuple();
            let unit = self.direction.primary(vertical, horizontal);

            let accumulator = match unit {
                AlignUnit::Start => &mut start_accumulation,
                AlignUnit::Middle => &mut middle_accumulation,
                AlignUnit::End => &mut end_accumulation,
            };

            *accumulator += accumulation + self.spacing;
        }

        let end_offset = primary - end_accumulation;

        let middle_center = (primary / 2.) - middle_accumulation / 2.;

        let start_end_bound = start_accumulation + self.spacing;
        let end_start_bound = end_offset - middle_accumulation - self.spacing;

        let middle_offset = (middle_center.max(start_end_bound)).min(end_start_bound);

        // There's no need for a start offset, it's always 0.
        (middle_offset, end_offset)
    }

    fn calculate_positions(
        &self,
        primary_accumulations: &Vec<Float>,
        secondary_accumulations: &Vec<Float>,
        children: &Vec<Element>,
        bounds: Dimensions,
    ) -> Vec<(Float, Float)> {
        let secondary_bounds = self.direction.secondary(bounds.width, bounds.height);

        let (middle_offset, end_offset) =
            self.calculate_position_offsets(primary_accumulations, children, bounds);

        let mut start_offset = 0.;
        let mut middle_offset = middle_offset;
        let mut end_offset = end_offset;

        fn increment(offset: &mut Float, value: Float) -> Float {
            let previous = *offset;
            *offset += value;

            previous
        }

        (0..children.len())
            .map(|i| {
                let child = &children[i];
                let accumulation = *&primary_accumulations[i];

                let (vertical, horizontal) = child.alignment().as_tuple();
                let (primary, secondary) = self.direction.swap(vertical, horizontal);

                let primary_offset = match primary {
                    AlignUnit::Start => increment(&mut start_offset, accumulation + self.spacing),
                    AlignUnit::Middle => increment(&mut middle_offset, accumulation + self.spacing),
                    AlignUnit::End => increment(&mut end_offset, accumulation + self.spacing),
                };

                let secondary_offset =
                    secondary.calculate(*&secondary_accumulations[i], secondary_bounds);

                (primary_offset, secondary_offset)
            })
            .collect()
    }

    fn calculate_childful(&self, element: &Element, outer: Rect) -> CalculatedElement {
        let inner = self.occupy_bounds(element, &outer);

        let children = element.children();

        let sorted_indices = {
            let mut indices: Vec<_> = (0..children.len()).collect();

            indices.sort_by_key(|i| {
                let child = &children[*i];

                let (width, height) = child.sizing().as_tuple();
                let sizing = self.direction.primary(width, height);

                sizing.index()
            });

            indices
        };

        let (primary_accumulations, secondary_accumulations) =
            self.calculate_accumulation(&sorted_indices, children, inner.clone());

        let mut calculated_children: Vec<Option<CalculatedElement>> = vec![None; children.len()];

        let (top, bottom, left, right) = element.padding().to_tuple();
        let (bounding_x, bounding_y) = outer.position.to_tuple();

        let positions = self.calculate_positions(
            &primary_accumulations,
            &secondary_accumulations,
            children,
            inner.dimensions,
        );

        // Get the secondary inner bound,
        // so that secondary stretching can be calculated
        let secondary_inner = {
            let (width, height) = inner.dimensions.to_tuple();
            let value = self.direction.secondary(width, height);

            value
        };

        let mut secondary_inner_accumulation = 0.;

        for i in 0..children.len() {
            let child = &children[i];

            let accumulation = &primary_accumulations[i];
            let (x, y) = &positions[i];

            let (width, height) = self.direction.swap(accumulation, &secondary_inner);
            let (x_offset, y_offset) = self.direction.swap(*x, *y);

            let final_x = bounding_x + x_offset + left;
            let final_y = bounding_y + y_offset + top;

            calculated_children[i] =
                Some(child.calculate(Rect::new(*width, *height, final_x, final_y)));

            // Add the secondary_inner_accumulation, only if this isn't a stretching element
            let (width, height) = child.sizing().as_tuple();
            let secondary_unit = self.direction.secondary(width, height);

            if secondary_unit != SizingUnit::Stretch {
                secondary_inner_accumulation =
                    secondary_accumulations[i].max(secondary_inner_accumulation);
            }
        }

        // Calculate the bounding box of this element with
        // the children + spacing
        let primary_bound: Float = primary_accumulations.iter().sum::<Float>()
            + ((children.len() - 1) as Float * self.spacing);

        // Account for padding in content
        let (width, height) = {
            let (w, h) = self
                .direction
                .swap(primary_bound, secondary_inner_accumulation);

            (w + left + right, h + top + bottom)
        };

        let calculated_bounds = element
            .sizing()
            .calculate(Dimensions::new(width, height), outer.dimensions);

        CalculatedElement {
            children: calculated_children.into_iter().flatten().collect(),
            rect: Rect::from_dimensions_and_position(calculated_bounds, outer.position),
        }
    }

    fn calculate_childless(&self, element: &Element, bounds: Rect) -> CalculatedElement {
        let calculated = element
            .sizing()
            .calculate_without_content(bounds.dimensions);

        let rect = Rect::from_dimensions_and_position(calculated, bounds.position);

        CalculatedElement::from_rect(rect)
    }

    pub fn calculate(&self, element: &Element, dimensions: Rect) -> CalculatedElement {
        if element.children().len() > 0 {
            return self.calculate_childful(element, dimensions);
        }

        self.calculate_childless(element, dimensions)
    }
}

trait DirectionalDimensions {
    fn occupy_with_direction(&mut self, direction: Direction, space_to_occupy: Float);
}

impl DirectionalDimensions for Dimensions {
    fn occupy_with_direction(&mut self, direction: Direction, space_to_occupy: Float) {
        let (directional, _) = direction.swap(&mut self.width, &mut self.height);
        *directional -= space_to_occupy;
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

        assert_eq!(result.rect.dimensions.width, 200.0);
        assert_eq!(result.rect.dimensions.height, 50.0);
    }

    #[test]
    fn calculates_stretch() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let element = directional(Horizontal, Stretch, Collapse, 0.)
            .children(vec![
                directional(Horizontal, Fixed(20.0), Stretch, 0.).build(),
                directional(Horizontal, Fixed(20.0), Fixed(30.0), 0.).build(),
                directional(Horizontal, Stretch, Stretch, 0.)
                    .label("stretcher")
                    .build(),
                directional(Horizontal, Fixed(20.0), Stretch, 0.).build(),
            ])
            .build();

        let result = element.calculate(rect);
        let child = &result.children[2];

        println!("{:?}", result);

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
