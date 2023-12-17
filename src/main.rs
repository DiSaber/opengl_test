mod program;
mod shader;
mod utils;
mod vector3;

use std::{ffi::CString, mem::size_of};

use glfw::Context;
use program::Program;
use shader::{Shader, ShaderType};
use vector3::Vector3;

extern crate gl;
extern crate glfw;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::ContextCreationApi(
        glfw::ContextCreationApi::Native,
    ));

    let (mut window, _events) = glfw
        .create_window(800, 600, "OpenGL Test", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));

    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

    let vertices: Vec<f32> = vec![
        Vector3::new(0.5, 0.5, 0.0),
        Vector3::new(0.5, -0.5, 0.0),
        Vector3::new(-0.5, -0.5, 0.0),
        Vector3::new(-0.5, 0.5, 0.0),
    ]
    .into_iter()
    .flatten()
    .collect();

    let indices: Vec<u32> = vec![Vector3::new(0, 1, 3), Vector3::new(1, 2, 3)]
        .into_iter()
        .flatten()
        .collect();

    let mut vbo = 0u32;
    let mut vao = 0u32;
    let mut ebo = 0u32;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * (size_of::<f32>() as i32),
            0 as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(0);
    }

    let vertex_shader = Shader::from_source(
        &CString::new(include_str!("shaders/triangle.vert")).unwrap(),
        ShaderType::VertexShader,
    )
    .unwrap();

    let fragment_shader = Shader::from_source(
        &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
        ShaderType::FragmentShader,
    )
    .unwrap();

    let mut shader_program = Program::new();

    shader_program.attach_shader(vertex_shader);
    shader_program.attach_shader(fragment_shader);
    shader_program.link_program().unwrap();

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.use_program();

        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                0 as *const std::ffi::c_void,
            )
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
