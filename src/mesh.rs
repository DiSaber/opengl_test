use crate::{texture::Texture, vertex::Vertex};

pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,
    total_vertices: i32,
}

impl Mesh {
    pub fn draw(&self, textures: &[&Texture]) {
        for (i, texture) in textures.iter().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + (i as u32));
                gl::BindTexture(gl::TEXTURE_2D, texture.get_id());
            }
        }

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

    pub fn from_buffer(object_buffer: &[f32], indices: &[u32]) -> Self {
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

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (object_buffer.len() * std::mem::size_of::<f32>()) as isize,
                object_buffer.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        let stride = Vertex::vec_size();
        let mut offset = 0;

        for (i, length) in Vertex::lengths().iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    *length as i32,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    offset as *const gl::types::GLvoid,
                );
                gl::EnableVertexAttribArray(i as u32);
            }

            offset += length * std::mem::size_of::<f32>();
        }

        mesh
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
