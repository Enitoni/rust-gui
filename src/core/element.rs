use crate::layout::Rect;

/// Represents an element returned from a component
/// with state and computed bounds
pub struct Element {
    rect: Rect,
    state: *const u8,
    children: Vec<Element>,
}
