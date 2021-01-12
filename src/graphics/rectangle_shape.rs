extern crate gl;

use std::mem;
use std::ptr;

use gl::types::*;

#[derive(Debug)]
pub struct RectangleShape {
    vbo: GLuint,
    vao: GLuint,
    pub width: f32,
    pub height: f32,
    pub left: f32,
    pub top: f32,
}

pub type Vertex = (f32, f32);

impl RectangleShape {
    pub fn new(width: f32, height: f32, left: f32, top: f32) -> RectangleShape {
        let mut r = RectangleShape {
            vbo: 0,
            vao: 0,
            width,
            height,
            left,
            top,
        };
        r.init_opengl_members();

        r
    }

    fn init_opengl_members(&mut self) {
        unsafe {
            gl::CreateVertexArrays(1, &mut self.vao);
            gl::CreateBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<GLfloat>() as i32 * 4,
                ptr::null(),
            );
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                mem::size_of::<GLfloat>() as i32 * 4,
                (mem::size_of::<GLfloat>() as i32 * 2) as *const _,
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }

        let mut verts: Vec<(Vertex, Vertex)> = Vec::new();

        for i in 0..4 {
            verts.push(self.get_point(i));
        }

        unsafe {
            gl::NamedBufferData(
                self.vbo,
                mem::size_of::<(Vertex, Vertex)>() as isize * verts.len() as isize,
                verts.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    fn get_point(&self, index: i32) -> (Vertex, Vertex) {
        let uv = match index {
            0 => (0.0, 0.0),
            1 => (1.0, 0.0),
            2 => (0.0, 1.0),
            3 => (1.0, 1.0),
            _ => panic!(),
        };
        let pos = (self.left + uv.0 * self.width, self.top + uv.1 * self.height);
        (pos, uv)
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}
