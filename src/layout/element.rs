use crate::{parsing::parse_sizing_unit, Alignment, ConstraintUnit};

use super::{
    alignment::*, calculated::CalculatedElement, common::*, dimension::Dimensions,
    directional::Directional, padding::Padding, rect::Rect,
};

#[derive(Debug)]
pub enum ElementKind {
    Directional(Directional),
    None,
}

#[derive(Debug)]
pub struct Element {
    kind: ElementKind,
    sizing: Sizing,
    padding: Padding,
    alignment: Alignment,
    children: Vec<Element>,
    label: Option<String>,
}

impl Element {
    pub fn calculate(&self, bounds: Rect) -> CalculatedElement {
        match &self.kind {
            ElementKind::Directional(l) => l.calculate(&self, bounds),
            ElementKind::None => CalculatedElement::empty(Dimensions::new(0.0, 0.0)),
        }
    }

    pub fn sizing(&self) -> &Sizing {
        &self.sizing
    }

    pub fn children(&self) -> &Vec<Element> {
        &self.children
    }

    pub fn padding(&self) -> &Padding {
        &self.padding
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }
}

pub struct ElementBuilder {
    kind: ElementKind,
    sizing: Sizing,
    padding: Padding,
    alignment: Alignment,
    children: Vec<Element>,
    label: Option<String>,
}

impl ElementBuilder {
    pub fn new() -> ElementBuilder {
        ElementBuilder {
            kind: ElementKind::None,
            children: Vec::new(),
            padding: Padding::empty(),
            alignment: Alignment::new(AlignUnit::Start, AlignUnit::Start),
            sizing: Sizing {
                width: SizingUnit::Collapse(ConstraintUnit::None),
                height: SizingUnit::Collapse(ConstraintUnit::None),
            },
            label: None,
        }
    }

    pub fn label(mut self, label: impl ToString) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn directional(mut self, direction: Direction, spacing: Float) -> Self {
        self.kind = ElementKind::Directional(Directional::new(direction, spacing));
        self
    }

    pub fn align(mut self, horizontal: AlignUnit, vertical: AlignUnit) -> Self {
        self.alignment = Alignment::new(horizontal, vertical);
        self
    }

    pub fn sizing(mut self, width: &str, height: &str) -> Self {
        self.sizing.width = parse_sizing_unit(width).unwrap();
        self.sizing.height = parse_sizing_unit(height).unwrap();
        self
    }

    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    pub fn pad_all(mut self, value: Float) -> Self {
        self.padding = Padding::new(value, value, value, value);
        self
    }

    pub fn build(self) -> Element {
        Element {
            kind: self.kind,
            label: self.label,
            sizing: self.sizing,
            padding: self.padding,
            children: self.children,
            alignment: self.alignment,
        }
    }
}
