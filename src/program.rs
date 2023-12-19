use std::ffi::{CStr, CString};

use crate::{
    shader::Shader, utils::WithLength, vector2::Vector2, vector3::Vector3, vector4::Vector4,
};

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

    pub fn set_value(&self, name: &CStr, value: ProgramValue) {
        let location = unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) };
        match value {
            ProgramValue::Float(float) => unsafe {
                gl::Uniform1f(location, float);
            },
            ProgramValue::Vec2(vec2) => unsafe {
                gl::Uniform2f(location, vec2.x, vec2.y);
            },
            ProgramValue::Vec3(vec3) => unsafe {
                gl::Uniform3f(location, vec3.x, vec3.y, vec3.z);
            },
            ProgramValue::Vec4(vec4) => unsafe {
                gl::Uniform4f(location, vec4.x, vec4.y, vec4.z, vec4.w);
            },
        };
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub enum ProgramValue {
    Float(f32),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Vec4(Vector4<f32>),
}
