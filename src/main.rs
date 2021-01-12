#[allow(dead_code)]
mod layout;
use std::time::Instant;

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

    rect.set_fill_color(Color::rgba(255, 255, 255, 10));

    rect.set_outline_color(Color::rgba(0, 110, 255, 60));
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

    println!("{:?}", element.calculate(Rect::from_dimensions(viewport)));

    window.set_active(true);

    let mut t = 0.;
    let mut i = 0;

    let mut time = Instant::now();

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

        t += time.elapsed().as_secs_f32();
        i += 1;

        time = Instant::now();

        if t > 1. {
            println!("FPS: {}", i);
            i = 0;
            t -= 1.;
        }

        window.clear(Color::BLACK);

        let size = window.size();
        let rect = Rect::new(size.x as f32, size.y as f32, 0.0, 0.0);

        let calculated = element.calculate(rect);
        let flattened = calculated.flatten();

        for child in flattened {
            window.draw(&create_rect_shape(child.rect.clone()));
        }

        window.display();
    }
}
