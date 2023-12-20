use std::ffi::{CStr, CString};

use crate::{
    shader::{Shader, ShaderType},
    utils::WithLength,
    vector2::Vector2,
    vector3::Vector3,
};

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
        let name = CString::new(name).unwrap();
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
        };
    }

    pub fn from_material_source(material_source: &str) {
        /*let vertex_shader = Shader::from_source(
            &material_source.vertex_shader_source,
            ShaderType::VertexShader,
        )
        .unwrap();

        let fragment_shader = Shader::from_source(
            &material_source.fragment_shader_source,
            ShaderType::FragmentShader,
        )
        .unwrap();

        let shader_program = Self::from_shaders(vec![&vertex_shader, &fragment_shader]).unwrap();

        for (name, value) in &material_source.program_values {
            shader_program.set_value(&name, *value)
        }*/
    }

    pub fn from_shaders(
        vertex_shader: &Shader,
        fragment_shader: &Shader,
        shader_values: Option<Vec<(&str, ProgramValue)>>,
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

        if let Some(shader_values) = shader_values {
            for (name, value) in shader_values {
                program.set_value(name, value)
            }
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
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
}
