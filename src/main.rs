mod vector3;

use glfw::Context;
use vector3::{FlattenVector3, Vector3};

extern crate gl;
extern crate glfw;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::ContextCreationApi(
        glfw::ContextCreationApi::Native,
    ));

    let (mut window, events) = glfw
        .create_window(960, 540, "OpenGL Test", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    gl::load_with(|s| window.get_proc_address(s));

    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

    let triangle_vertices = vec![
        Vector3 {
            x: -0.5,
            y: -0.5,
            z: 0.0,
        },
        Vector3 {
            x: 0.5,
            y: -0.5,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
    ]
    .flatten();

    let mut vbo: u32 = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        )
    }

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
