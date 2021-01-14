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
    pub x: f32,
    pub y: f32,
    pub border_thickness: f32,
}

impl Drop for RectangleShape {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}

pub type Vertex = (f32, f32);

impl RectangleShape {
    pub fn new(
        width: f32,
        height: f32,
        left: f32,
        top: f32,
        border_thickness: Option<f32>,
    ) -> RectangleShape {
        let mut r = RectangleShape {
            vbo: 0,
            vao: 0,
            width,
            height,
            x: left,
            y: top,
            border_thickness: border_thickness.unwrap_or(0.),
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

        // borders
        if self.border_thickness != 0. {
            let corners = [
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ];

            let corners_extruded = [
                (
                    self.x - self.border_thickness,
                    self.y - self.border_thickness,
                ),
                (
                    self.x + self.width + self.border_thickness,
                    self.y - self.border_thickness,
                ),
                (
                    self.x - self.border_thickness,
                    self.y + self.height + self.border_thickness,
                ),
                (
                    self.x + self.width + self.border_thickness,
                    self.y + self.height + self.border_thickness,
                ),
            ];

            let border_verts = [
                //left
                corners_extruded[0],
                corners[0],
                corners_extruded[2],
                corners[0],
                corners[2],
                corners_extruded[2],
                // top
                corners_extruded[0],
                corners_extruded[1],
                corners[0],
                corners[0],
                corners_extruded[1],
                corners[1],
                // right
                corners[1],
                corners_extruded[1],
                corners[3],
                corners[3],
                corners_extruded[1],
                corners_extruded[3],
                // bottom
                corners[2],
                corners[3],
                corners_extruded[2],
                corners_extruded[2],
                corners[3],
                corners_extruded[3],
            ];

            verts.extend(border_verts.into_iter().map(|v| (*v, (0., 0.))));
        }

        unsafe {
            // initialize just the vertex data
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
        let pos = (self.x + uv.0 * self.width, self.y + uv.1 * self.height);
        (pos, uv)
    }

    pub fn draw(&self, shader: &super::shader::Shader) {
        unsafe {
            gl::BindVertexArray(self.vao);
            shader.setUniform("is_border", 0);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            if self.border_thickness != 0. {
                shader.setUniform("is_border", 1);
                gl::DrawArrays(gl::TRIANGLES, 4, 24);
            }

            gl::BindVertexArray(0);
        }
    }
}
