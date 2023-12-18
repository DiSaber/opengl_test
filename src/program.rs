use std::ffi::CString;

use crate::{shader::Shader, utils::WithLength};

pub struct Program {
    id: u32,
}

impl Program {
    pub fn from_shaders(shaders: Vec<&Shader>) -> Result<Self, CString> {
        let program = Program {
            id: unsafe { gl::CreateProgram() },
        };

        for shader in &shaders {
            unsafe {
                gl::AttachShader(program.id, shader.get_id());
            }
        }

        unsafe {
            gl::LinkProgram(program.id);
        }

        let mut success = 0i32;
        unsafe {
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut error_length = 0i32;
            unsafe {
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let error = CString::with_length(error_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program.id,
                    error_length,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut i8,
                )
            }

            return Err(error);
        }

        for shader in &shaders {
            unsafe {
                gl::DetachShader(program.id, shader.get_id());
            }
        }

        Ok(program)
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
