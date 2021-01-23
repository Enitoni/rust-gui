use super::{
    alignment::*, calculated::CalculatedElement, common::*, dimension::Dimensions,
    element::Element, rect::Rect,
};

#[derive(Debug)]
/// Represents a layout of child elements in a given direction, with a given spacing
pub struct Directional {
    /// Direction of layout
    direction: Direction,

    /// Spacing between children (by default 0)
    spacing: Float,
}

impl Directional {
    pub fn new(direction: Direction, spacing: Float) -> Directional {
        Directional { direction, spacing }
    }

    fn calculate_position_offsets(
        &self,
        accumulations: &Vec<Float>,
        children: &Vec<Element>,
        inner_bounds: &Rect,
    ) -> (f32, f32) {
        let (width, height) = inner_bounds.dimensions.as_tuple();
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
        element: &Element,
        primary_accumulations: &Vec<Float>,
        secondary_accumulations: &Vec<Float>,
        inner_bounds: &Rect,
    ) -> Vec<(Float, Float)> {
        let children = element.children();
        let (offset_x, offset_y) = inner_bounds.position.as_tuple();

        let secondary_bounds = {
            let (width, height) = inner_bounds.dimensions.as_tuple();
            self.direction.secondary(width, height)
        };

        let (middle_offset, end_offset) =
            self.calculate_position_offsets(primary_accumulations, children, inner_bounds);

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

                let (x, y) = self.direction.swap(primary_offset, secondary_offset);

                (offset_x + x, offset_y + y)
            })
            .collect()
    }

    fn calculate_inner_bounds(&self, element: &Element, box_bounds: &Rect) -> Rect {
        let (top, bottom, left, right) = element.padding().as_tuple();

        let (mut width, mut height) = box_bounds.dimensions.as_tuple();
        let (mut x, mut y) = box_bounds.position.as_tuple();

        width -= left + right;
        height -= top + bottom;

        x += left;
        y += top;

        Rect::new(width, height, x, y)
    }

    fn sort_primary_indices(&self, element: &Element) -> Vec<usize> {
        let children = element.children();
        let mut indices: Vec<_> = (0..children.len()).collect();

        indices.sort_by_key(|i| {
            let child = &children[*i];

            let (width, height) = child.sizing().as_tuple();
            let sizing = self.direction.primary(width, height);

            sizing.index()
        });

        indices
    }

    // Loops through the children to get the accumulated space needed for the final calculation
    fn calculate_accumulation(
        &self,
        sorted_indices: &Vec<usize>,
        inner_bounds: &Rect,
        children: &Vec<Element>,
    ) -> (Vec<f32>, Vec<f32>, Float, Float) {
        let mut primary_intrinsic: Float = 0.;
        let mut primary_accumulation: Vec<Float> = vec![0.; children.len()];

        let mut secondary_intrinsic: Float = 0.;
        let mut secondary_accumulation: Vec<Float> = vec![0.; children.len()];

        let (width, height, _, _) = inner_bounds.as_tuple();
        let (available_primary, available_secondary) = self.direction.swap(width, height);

        // Occupy spacing between children
        let available_primary = available_primary - (children.len() - 1) as Float * self.spacing;

        fn calculate_intrinsic(child: &Element, inner_bounds: Rect) -> (Float, Float) {
            child
                // Inner bounds is used for both designated and outer, because
                // designated space is not known at this time
                .calculate(inner_bounds.clone(), inner_bounds)
                .rect
                .dimensions
                .as_tuple()
        }

        fn calculate_stretch(
            child: &Element,
            primary_intrinsic: Float,
            available_primary: Float,
            available_secondary: Float,
            direction: Direction,
        ) -> (Float, Float) {
            let available = available_primary - primary_intrinsic;
            let (width, height) = direction.swap(available, available_secondary);

            let available = Rect::new(width, height, 0.0, 0.0);

            child
                .calculate(available.clone(), available)
                .rect
                .dimensions
                .as_tuple()
        }

        for index in sorted_indices {
            let child = &children[*index];
            let child_sizing = child.sizing();

            // Get the primary (directional) unit here
            let (primary_unit, secondary_unit) = self
                .direction
                .swap(&child_sizing.width, &child_sizing.height);

            let (calculated_width, calculated_height) = match primary_unit {
                SizingUnit::Fixed(_) | SizingUnit::Collapse(_) | SizingUnit::Percent(_, _, _) => {
                    calculate_intrinsic(child, inner_bounds.clone())
                }
                SizingUnit::Stretch(_) => calculate_stretch(
                    child,
                    primary_intrinsic,
                    available_primary,
                    available_secondary,
                    self.direction,
                ),
            };

            let (primary, secondary) = self.direction.swap(calculated_width, calculated_height);

            primary_accumulation[*index] = primary;
            primary_intrinsic += primary;

            secondary_accumulation[*index] = secondary;

            // Accumulate intrinsic only for collapse/fixed,
            // So stretching secondaries can stretch based on that
            match secondary_unit {
                SizingUnit::Stretch(_) => {}
                _ => secondary_intrinsic = secondary_intrinsic.max(secondary),
            }
        }

        (
            primary_accumulation,
            secondary_accumulation,
            primary_intrinsic,
            secondary_intrinsic,
        )
    }

    fn calculate_box_bounds(
        &self,
        element: &Element,
        available_bounds: &Rect,
        outer_bounds: &Rect,
        primary_intrinsic: Float,
        secondary_intrinsic: Float,
    ) -> Rect {
        let inner = {
            // Add the missing spacing to the intrinsic
            let primary_intrinsic =
                primary_intrinsic + (element.children().len() - 1) as Float * self.spacing;

            let (width, height) = self.direction.swap(primary_intrinsic, secondary_intrinsic);
            let (top, bottom, left, right) = element.padding().as_tuple();

            Dimensions::new(width + left + right, height + top + bottom)
        };

        Rect::from_dimensions_and_position(
            element
                .sizing()
                .calculate(inner, available_bounds.dimensions, outer_bounds.dimensions),
            available_bounds.position,
        )
    }

    fn calculate_childful(
        &self,
        element: &Element,
        designated_bounds: Rect,
        outer_bounds: Rect,
    ) -> CalculatedElement {
        let box_bounds =
            self.calculate_box_bounds(element, &designated_bounds, &outer_bounds, 0., 0.);

        let inner_bounds = self.calculate_inner_bounds(element, &box_bounds);
        let sorted_indices = self.sort_primary_indices(element);

        //dbg!(element.label(), &box_bounds, &inner_bounds, &outer_bounds);

        let (
            primary_accumulations,
            secondary_accumulations,
            primary_intrinsic,
            secondary_intrinsic,
        ) = self.calculate_accumulation(&sorted_indices, &inner_bounds, element.children());

        // Calculate the new box and inner bounds so future calculations are correct
        let box_bounds = self.calculate_box_bounds(
            element,
            &designated_bounds,
            &outer_bounds,
            primary_intrinsic,
            secondary_intrinsic,
        );

        let inner_bounds = self.calculate_inner_bounds(element, &box_bounds);

        let positions = self.calculate_positions(
            &element,
            &primary_accumulations,
            &secondary_accumulations,
            &inner_bounds,
        );

        let children = element.children();
        let mut calculated_children: Vec<Option<CalculatedElement>> = vec![None; children.len()];

        for i in 0..children.len() {
            let child = &children[i];

            let primary = &primary_accumulations[i];
            let secondary = &secondary_accumulations[i];

            let (outer_width, outer_height) = self.direction.swap(primary, secondary);
            let (x, y) = &positions[i];

            calculated_children[i] = Some(child.calculate(
                Rect::new(*outer_width, *outer_height, *x, *y),
                inner_bounds.clone(),
            ));
        }

        CalculatedElement {
            children: calculated_children.into_iter().flatten().collect(),
            rect: box_bounds,
        }
    }

    fn calculate_childless(
        &self,
        element: &Element,
        available_bounds: Rect,
        outer_bounds: Rect,
    ) -> CalculatedElement {
        let calculated = element
            .sizing()
            .calculate_without_content(available_bounds.dimensions, outer_bounds.dimensions);

        //dbg!(element.label(), &available_bounds, &outer_bounds);

        let rect = Rect::from_dimensions_and_position(calculated, available_bounds.position);

        CalculatedElement::from_rect(rect)
    }

    pub fn calculate(
        &self,
        element: &Element,
        available_bounds: Rect,
        outer_bounds: Rect,
    ) -> CalculatedElement {
        if element.children().len() > 0 {
            return self.calculate_childful(element, available_bounds, outer_bounds);
        }

        self.calculate_childless(element, available_bounds, outer_bounds)
    }
}

#[cfg(test)]
mod test {
    use crate::layout::{element::ElementBuilder, rect::Rect, Direction::*};

    #[test]
    fn calculates_childless() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let a = ElementBuilder::new()
            .directional(Horizontal, 0.0)
            .sizing("Fixed:50", "Stretch")
            .build();

        let result = a.calculate(rect.clone(), rect);

        assert_eq!(result.rect.dimensions.width, 50.0);
        assert_eq!(result.rect.dimensions.height, 100.0);
    }

    #[test]
    fn calculates_collapse() {
        let rect = Rect::new(200.0, 200.0, 0.0, 0.0);

        let child = ElementBuilder::new()
            .directional(Horizontal, 0.0)
            .sizing("Fixed:50", "Fixed:50")
            .build();

        let a = ElementBuilder::new()
            .directional(Vertical, 0.0)
            .sizing("Stretch", "Collapse")
            .children(vec![child])
            .build();

        let result = a.calculate(rect.clone(), rect);

        assert_eq!(result.rect.dimensions.width, 200.0);
        assert_eq!(result.rect.dimensions.height, 50.0);
    }

    #[test]
    fn calculates_stretch() {
        let rect = Rect::new(100.0, 100.0, 0.0, 0.0);

        let element = ElementBuilder::new()
            .directional(Horizontal, 0.)
            .sizing("Stretch", "Collapse")
            .children(vec![
                ElementBuilder::new()
                    .directional(Horizontal, 0.)
                    .sizing("Fixed:20", "Stretch")
                    .build(),
                ElementBuilder::new()
                    .directional(Horizontal, 0.)
                    .sizing("Fixed:20", "Fixed:30")
                    .build(),
                ElementBuilder::new()
                    .directional(Horizontal, 0.)
                    .sizing("Stretch", "Stretch")
                    .label("stretcher")
                    .build(),
                ElementBuilder::new()
                    .directional(Horizontal, 0.)
                    .sizing("Fixed:20", "Stretch")
                    .build(),
            ])
            .build();

        let result = element.calculate(rect.clone(), rect);
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

        let element = ElementBuilder::new()
            .directional(Horizontal, 0.)
            .sizing("Stretch", "Fixed:50")
            .children(vec![ElementBuilder::new()
                .directional(Horizontal, 0.)
                .sizing("Stretch", "Stretch")
                .build()])
            .pad_all(10.0)
            .build();

        let result = element.calculate(rect.clone(), rect);
        let child = &result.children[0];

        assert_eq!(child.rect.dimensions.height, 50.0 - (10. * 2.0));
        assert_eq!(child.rect.dimensions.width, 100.0 - (10. * 2.0));
    }
}
