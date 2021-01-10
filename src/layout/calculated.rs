use std::fmt;

use super::{dimension::Dimensions, rect::Rect};

const INDENTATION: &str = "  ";

pub struct CalculatedElement {
    pub rect: Rect,
    pub children: Vec<CalculatedElement>,
}

impl CalculatedElement {
    pub fn empty(dimensions: Dimensions) -> CalculatedElement {
        CalculatedElement {
            rect: Rect::from_dimensions(dimensions),
            children: Vec::new(),
        }
    }

    pub fn from_rect(rect: Rect) -> CalculatedElement {
        CalculatedElement {
            rect,
            children: Vec::new(),
        }
    }

    pub fn display(&self) -> String {
        self.stringify(0)
    }

    pub fn nodes(&self) -> usize {
        self.children.iter().fold(1, |acc, c| acc + c.nodes())
    }

    fn stringify(&self, indents: usize) -> String {
        let CalculatedElement { rect, .. } = self;
        let Rect {
            dimensions,
            position,
        } = rect;

        let head = format!(
            "{}<width={} height={} x={} y={}",
            INDENTATION.repeat(indents),
            dimensions.width,
            dimensions.height,
            position.x,
            position.y,
        );

        if self.children.len() == 0 {
            format!("{}/>", head)
        } else {
            let mapped_children: &String = &self
                .children
                .iter()
                .map(|c| format!("\n{}", c.stringify(indents + 1)))
                .collect::<Vec<_>>()
                .join("");

            format!(
                "{}>{}\n{}</>",
                head,
                mapped_children,
                INDENTATION.repeat(indents)
            )
        }
    }
}

impl fmt::Debug for CalculatedElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}
