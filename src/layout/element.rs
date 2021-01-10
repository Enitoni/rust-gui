use super::{
    calculated::CalculatedElement, common::*, dimension::Dimensions, directional::Directional,
    rect::Rect,
};

pub enum ElementKind {
    Directional(Directional),
    None,
}

pub struct Element {
    kind: ElementKind,
    children: Vec<Element>,
    sizing: Sizing,
}

impl Element {
    pub fn calculate(&self, bounds: Option<Rect>) -> CalculatedElement {
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
}

pub struct ElementBuilder {
    kind: ElementKind,
    children: Vec<Element>,
    sizing: Sizing,
}

impl ElementBuilder {
    pub fn new() -> ElementBuilder {
        ElementBuilder {
            kind: ElementKind::None,
            children: Vec::new(),
            sizing: Sizing {
                width: SizingUnit::Collapse,
                height: SizingUnit::Collapse,
            },
        }
    }

    pub fn directional(mut self, directional: Directional) -> Self {
        self.kind = ElementKind::Directional(directional);
        self
    }

    pub fn sizing(mut self, width: SizingUnit, height: SizingUnit) -> Self {
        self.sizing.width = width;
        self.sizing.height = height;
        self
    }

    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> Element {
        Element {
            kind: self.kind,
            children: self.children,
            sizing: self.sizing,
        }
    }
}
