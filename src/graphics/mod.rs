pub mod rectangle_shape;
pub mod shader;

use gl::types::GLuint;
use shader::Uniform;

use std::ffi::CString;

impl Uniform for i32 {
    fn set(&self, id: &str, handle: GLuint) {
        unsafe {
            let name = CString::new(id.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(handle, name.as_ptr());
            gl::ProgramUniform1i(handle, location, *self);
        }
    }
}

impl Uniform for f32 {
    fn set(&self, id: &str, handle: GLuint) {
        unsafe {
            let name = CString::new(id.as_bytes()).unwrap();
            let location = gl::GetUniformLocation(handle, name.as_ptr());
            gl::ProgramUniform1f(handle, location, *self);
        }
    }
}
