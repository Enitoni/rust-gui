extern crate gl;

use std::mem;
use std::ptr;

use gl::types::*;

use crate::Float;
pub type Float4 = (Float, Float, Float, Float);

#[derive(Debug)]
pub struct RGBATexture {
    pub handle: GLuint,
}

impl RGBATexture {
    pub fn new(width: usize, height: usize, data: *const u8) -> Self {
        let mut handle: GLuint = 0;

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut handle);

            gl::TextureStorage2D(handle, 1, gl::RGBA8, width as i32, height as i32);
            gl::TextureSubImage2D(
                handle,
                0,
                0,
                0,
                width as i32,
                height as i32,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const GLvoid,
            );

            gl::TextureParameteri(handle, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TextureParameteri(handle, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TextureParameteri(handle, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(handle, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        RGBATexture { handle }
    }
}

impl Drop for RGBATexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.handle);
        }
    }
}

#[derive(Debug)]
pub struct RectangleShape {
    vbo: GLuint,
    vao: GLuint,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub border_thickness: f32,
    pub border_color: Float4,
    pub fill_color: Float4,
    pub texture: Option<RGBATexture>,
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
        fill_color: Float4,
        border_color: Float4,
        texture: Option<RGBATexture>,
    ) -> RectangleShape {
        let mut r = RectangleShape {
            vbo: 0,
            vao: 0,
            width,
            height,
            x: left,
            y: top,
            border_thickness: border_thickness.unwrap_or(0.),
            border_color,
            fill_color,
            texture,
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

            if let Some(tex) = &self.texture {
                shader.setUniform("has_texture", 1);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, tex.handle);
            } else {
                shader.setUniform("has_texture", 0);
            }

            shader.setUniform("is_border", 0);
            shader.setUniform("fill_color", self.fill_color);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

            if self.border_thickness != 0. {
                shader.setUniform("is_border", 1);
                shader.setUniform("border_color", self.border_color);
                gl::DrawArrays(gl::TRIANGLES, 4, 24);
            }

            gl::BindVertexArray(0);
        }
    }
}
