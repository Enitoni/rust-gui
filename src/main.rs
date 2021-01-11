#[allow(dead_code)]
mod layout;
pub use layout::*;

mod mock;
pub use mock::*;

use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View},
    system::*,
    window::*,
};

use mock::layout::*;

// Returns a testing rect shape
fn create_rect_shape<'a>(rect: layout::Rect) -> RectangleShape<'a> {
    let (width, height, x, y) = rect.to_tuple();

    let mut rect = RectangleShape::new();

    rect.set_size((width, height));
    rect.set_position((x, y));

    rect.set_fill_color(Color::rgba(65, 190, 245, 20));

    rect.set_outline_color(Color::rgba(75, 235, 180, 50));
    rect.set_outline_thickness(-1.0);

    rect
}

fn main() {
    let viewport = Dimensions::new(800.0, 800.0);

    let mut window = RenderWindow::new(
        (800, 800),
        "Layout test",
        Style::DEFAULT,
        &Default::default(),
    );

    let element = test_layout();

    println!(
        "{:?}",
        element.calculate(Some(Rect::from_dimensions(viewport)))
    );

    window.set_active(true);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    let width = width as f32;
                    let height = height as f32;

                    let new_view = View::new(
                        Vector2f::new(width / 2.0, height / 2.0),
                        Vector2f::new(width, height),
                    );

                    window.set_view(&new_view);
                }
                _ => (),
            }
        }

        window.clear(Color::BLACK);

        let size = window.size();
        let rect = Rect::new(size.x as f32, size.y as f32, 0.0, 0.0);

        let calculated = element.calculate(Some(rect));
        let flattened = calculated.flatten();

        for child in flattened {
            window.draw(&create_rect_shape(child.rect.clone()));
        }

        window.display();
    }
}
