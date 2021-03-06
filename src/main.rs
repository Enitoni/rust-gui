#![feature(box_syntax)]

#[allow(dead_code)]
mod layout;
use std::time::Instant;

mod parsing;

pub use layout::*;

mod mock;
pub use mock::*;

use mock::layout::*;

use gl::types::*;

pub mod graphics;
use graphics::rectangle_shape::{Float4, RGBATexture, RectangleShape};

fn create_rect_shape(rect: &layout::Rect) -> RectangleShape {
    let (width, height, x, y) = rect.as_tuple();

    RectangleShape::new(
        width,
        height,
        x,
        y,
        Some(-1.0),
        (0.3, 0.3, 1.0, 0.04),
        (1.2, 0.2, 1.0, 0.3),
        None,
    )
}

// // Returns a testing rect shape
// fn create_rect_shape<'a>(rect: layout::Rect) -> RectangleShape<'a> {
//     let (width, height, x, y) = rect.to_tuple();

//     let mut rect = RectangleShape::new();

//     rect.set_size((width, height));
//     rect.set_position((x, y));

//     rect.set_fill_color(Color::rgba(255, 255, 255, 10));

//     rect.set_outline_color(Color::rgba(0, 110, 255, 60));
//     rect.set_outline_thickness(-1.0);

//     rect
// }

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use rusttype::VMetrics;

fn main() {
    let viewport = Dimensions::new(800.0, 800.0);

    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Layout test");
    let windowed_context = ContextBuilder::new()
        .with_vsync(false)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    windowed_context
        .window()
        .set_min_inner_size(Some(glutin::dpi::PhysicalSize::new(780, 650)));

    &windowed_context
        .window()
        .set_inner_size(glutin::dpi::PhysicalSize::new(800, 800));

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let element = test_layout();

    println!(
        "{:?}",
        element.calculate(
            Rect::from_dimensions(viewport),
            Rect::from_dimensions(viewport)
        )
    );

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    let box_shader = graphics::shader::Shader::new();
    box_shader
        .attach(
            &std::fs::read_to_string("assets/box.vs").unwrap(),
            gl::VERTEX_SHADER,
        )
        .unwrap();
    box_shader
        .attach(
            &std::fs::read_to_string("assets/box.fs").unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();
    box_shader.compile().unwrap();
    box_shader.bind();

    let mut t = 0;
    let mut i = 0;

    let mut time = Instant::now();
    let mut size = windowed_context.window().inner_size();

    // some font rendering tests

    let font_data = std::fs::read("./assets/fonts/DejaVuSans.ttf").unwrap();
    let font = rusttype::Font::try_from_bytes(&font_data).expect("Error constructing font");

    let test_string = "kroße krabbe pizza ist die pizza für dich und mich".chars();
    let mut test_chars = test_string.map(|c| (c, font.glyph(c)));

    let scale = rusttype::Scale::uniform(24.);
    let start = rusttype::vector(200., 200.);

    const RED: Float4 = (1.0, 0.0, 0.0, 1.0);
    const TRANSPARENT: Float4 = (0.0, 0.0, 0.0, 0.0);

    let MAX_WIDTH = 140.;

    // last point in the string where a line-wrap is desirable
    let mut last_breakpoint = None;
    // stored glyphs
    let mut calculated_glyphs = Vec::new();

    // last glyph on this line, for calculating kerning
    let mut last_glyph = None;
    let mut x_offset = 0.;
    let mut y_offset = 0.;

    while let Some((c, g)) = test_chars.next() {
        if c.is_whitespace() {
            last_breakpoint = Some((test_chars.clone(), calculated_glyphs.len()));
        }

        let g = g.scaled(scale);
        if let Some(last) = last_glyph {
            x_offset += font.pair_kerning(scale, last, g.id());
        }

        let w = g.h_metrics().advance_width;
        let next = g.positioned(start + rusttype::point(x_offset, y_offset));
        last_glyph = Some(next.id());
        x_offset += w;
        calculated_glyphs.push(next);

        if x_offset > MAX_WIDTH {
            x_offset = 0.;
            let v_metrics = font.v_metrics(scale);
            y_offset += (v_metrics.ascent - v_metrics.descent) + v_metrics.line_gap;
            last_glyph = None;
            if let Some((last, count)) = last_breakpoint.take() {
                test_chars = last;
                calculated_glyphs.truncate(count);
            }
        }
    }

    let text = {
        let rects = calculated_glyphs
            .iter()
            .flat_map(|g| {
                let b = g.pixel_bounding_box()?;
                let mut buffer: Vec<(u8, u8, u8, u8)> =
                    vec![(0, 0, 0, 0); b.width() as usize * b.height() as usize];

                g.draw(|x, y, v| {
                    buffer[x as usize + y as usize * b.width() as usize] =
                        (255, 0, 0, (v * 255.) as u8);
                });

                Some(RectangleShape::new(
                    b.width() as f32,
                    b.height() as f32,
                    b.min.x as f32,
                    b.min.y as f32,
                    None,
                    RED,
                    TRANSPARENT,
                    Some(RGBATexture::new(
                        b.width() as usize,
                        b.height() as usize,
                        buffer.as_ptr() as *const u8,
                    )),
                ))
            })
            .collect::<Vec<_>>();

        rects
    };

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    size = windowed_context.window().inner_size();
                    unsafe {
                        gl::Viewport(
                            0,
                            0,
                            physical_size.width as i32,
                            physical_size.height as i32,
                        );
                    }
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                // Application update code.
                windowed_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                t += time.elapsed().as_millis();
                i += 1;

                time = Instant::now();

                if t > 1000 {
                    println!("FPS: {}", i);
                    i = 0;
                    t = 0;
                }

                let rect = Rect::new(size.width as f32, size.height as f32, 0.0, 0.0);

                let calculated = element.calculate(rect.clone(), rect);
                let flattened = calculated.flatten();

                unsafe {
                    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                box_shader.bind();
                box_shader.setUniform("fb_width", size.width as f32);
                box_shader.setUniform("fb_height", size.height as f32);

                for child in flattened {
                    let rect = create_rect_shape(&child.rect);
                    rect.draw(&box_shader);
                }

                for r in text.iter() {
                    r.draw(&box_shader);
                }

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
