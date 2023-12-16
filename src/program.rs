use std::ffi::CString;

use crate::{shader::Shader, utils::WithLength};

pub struct Program {
    pub id: u32,
    shaders: Vec<Shader>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            id: unsafe { gl::CreateProgram() },
            shaders: Vec::new(),
        }
    }

    pub fn attach_shader(&mut self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }

        self.shaders.push(shader);
    }

    fn detach_shader(&self, shader: &Shader) {
        unsafe {
            gl::DetachShader(self.id, shader.id);
        }
        println!("Detatched");
    }

    pub fn link_program(&mut self) -> Result<(), CString> {
        unsafe {
            gl::LinkProgram(self.id);
        }

        let mut success = 0i32;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut error_length = 0i32;
            unsafe {
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let error = CString::with_length(error_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    self.id,
                    error_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut i8,
                )
            }

            return Err(error);
        }

        for shader in &self.shaders {
            self.detach_shader(shader);
        }

        self.shaders.clear();

        Ok(())
    }
}
