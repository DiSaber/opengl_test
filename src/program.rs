use std::ffi::CString;

use crate::{shader::Shader, utils::WithLength};

pub struct Program {
    id: u32,
}

impl Program {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_value(&self, name: &str, value: ProgramValue) {
        self.use_program();
        let name = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) };
        match value {
            ProgramValue::Float(float) => unsafe {
                gl::Uniform1f(location, float);
            },
            ProgramValue::Int(int) => unsafe {
                gl::Uniform1i(location, int);
            },
        };
    }

    pub fn from_shaders(
        vertex_shader: &Shader,
        fragment_shader: &Shader,
        shader_values: &[(&str, ProgramValue)],
    ) -> Result<Self, CString> {
        let program = Program {
            id: unsafe { gl::CreateProgram() },
        };

        unsafe {
            gl::AttachShader(program.id, vertex_shader.get_id());
            gl::AttachShader(program.id, fragment_shader.get_id());
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

        unsafe {
            gl::DetachShader(program.id, vertex_shader.get_id());
            gl::DetachShader(program.id, fragment_shader.get_id());
        }

        for (name, value) in shader_values {
            program.set_value(name, *value)
        }

        Ok(program)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProgramValue {
    Float(f32),
    Int(i32),
}
