use rust_gui::{Alignment, Padding, Rect, Sizing};

use super::element::Element;


struct LayoutContext {
    current_node: usize,
    outer_bounds: Rect,
    tree: Rc<Tree>,
}

struct LayoutProperties {
    pub(crate) sizing: Sizing,
    pub(crate) padding: Padding,
    pub(crate) alignment: Alignment
}


trait Component {
    type State: Default;

    fn initial_state() -> Self::State {
        Self::State::default()
    }

    fn allow_children() -> bool { false }
    
    // how should the parent of this component treat this component
    fn layout_properties(&self) -> LayoutProperties;

    // render self and children
    fn render_self(&self, ctx: LayoutContext, state: &mut Self::State) -> Rect;
    
    // do we need to recompute
    fn needs_reflow(&self, state: &Self::State) -> bool;
}

/// Workaround for not being able to
/// make a reference to a tree with an associated type
trait ErasedComponent {
    unsafe fn render(&self, state: *mut u8) -> Rect;
    unsafe fn needs_reflow(&self, data: *const u8) -> bool;
}

impl<T> ErasedComponent for T
where
    T: Component,
{
    unsafe fn render(&self, state: *mut u8) -> Rect {
        Component::render(self, &mut *(state as *mut T::State))
    }

    unsafe fn needs_reflow(&self, state: *const u8) -> bool {
        Component::needs_reflow(self, &*(state as *const T::State))
    }
}
