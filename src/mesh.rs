use std::mem::size_of;

use crate::buffer::Buffer;

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    total_vertices: i32,
}

impl Mesh {
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.total_vertices,
                gl::UNSIGNED_INT,
                0 as *const std::ffi::c_void,
            )
        }
    }

    pub fn from_buffers(
        object_buffer: Vec<Buffer<f32>>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        let Some(first) = object_buffer.get(0) else {
            return Err("The object buffer cannot be empty!");
        };
        let length = first.buffer.len() / first.size;

        if let Some(slice) = object_buffer.get(1..) {
            for buffer in slice {
                assert_eq!(
                    buffer.buffer.len() / buffer.size,
                    length,
                    "Buffers must be of equal length!"
                );
            }
        }

        let mut mesh = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            total_vertices: indices.len() as i32,
        };
        unsafe {
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::GenVertexArrays(1, &mut mesh.vao);

            gl::BindVertexArray(mesh.vao);
        }

        let mut final_buffer: Vec<f32> = Vec::with_capacity(
            object_buffer
                .iter()
                .fold(0, |acc, buffer| acc + buffer.buffer.len()),
        );

        for i in 0..length {
            for buffer in &object_buffer {
                final_buffer.extend_from_slice(
                    buffer
                        .buffer
                        .get((buffer.size * i)..(buffer.size * (i + 1)))
                        .unwrap(),
                )
            }
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (final_buffer.len() * size_of::<f32>()) as isize,
                final_buffer.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as isize,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        let stride = object_buffer
            .iter()
            .fold(0, |acc, buffer| acc + buffer.size)
            * size_of::<f32>();
        let mut offset = 0;

        for (i, buffer) in object_buffer.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    buffer.size as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(i as u32);
            }

            offset += buffer.size * size_of::<f32>();
        }

        Ok(mesh)
    }
}
