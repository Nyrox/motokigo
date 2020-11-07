extern crate gl;

use std::ffi::CString;
use std::ptr;
use std::str;

use self::gl::types::*;

#[derive(Debug)]
pub struct Shader {
    handle: GLuint,
}

pub trait Uniform {
    fn set(&self, id: &str, handle: GLuint);
}

// This class name lies, its really a ShaderProgram
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}

impl Shader {
    pub fn new() -> Shader {
        unsafe {
            Shader {
                handle: gl::CreateProgram(),
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn attach(&self, source: &str, shader_type: GLenum) -> Result<(), String> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                return Err(String::from_utf8(buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8"));
            }

            gl::AttachShader(self.handle, shader);
            gl::DeleteShader(shader);
        }

        Ok(())
    }

    pub fn compile(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.handle);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(self.handle, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetProgramiv(self.handle, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    self.handle,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );

                return Err(String::from_utf8(buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8"));
            }
        }
        Ok(())
    }

    pub fn set_uniform<T>(&self, id: &str, val: T)
    where
        T: Uniform,
    {
        val.set(id, self.handle);
    }
}
