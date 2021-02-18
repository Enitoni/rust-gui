use super::element::Element;

trait Component {
    type State;

    fn render(&self) -> Element;
    fn needs_reflow(&self, old: &Element) {}
}

/// Workaround for not being able to
/// make a reference to a tree with an associated type
trait ErasedComponent {
    fn compute(&self) -> (ComputedElement, *const u8);
    unsafe fn needs_reflow(&self, data: *const u8) -> bool;
}

impl<T> ErasedComponent for T
where
    T: Component,
{
    fn compute(&self) -> (ComputedElement, *const u8) {
        let (elem, data) = Element::render(self);
        let data = Box::new(data);

        (elem, Box::into_raw(data) as *const u8)
    }

    unsafe fn needs_reflow(&self, data: *const u8) -> bool {
        Element::needs_reflow(self, &*(data as *const <Self as Element>::ComputedData))
    }
}
