mod buffer;
mod mesh;
mod program;
mod shader;
mod utils;
mod vector2;
mod vector3;

use buffer::Buffer;
use glfw::Context;
use mesh::Mesh;
use program::{Program, ProgramValue};
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

    let vertices = Buffer::new(
        vec![
            Vector3::new(0.5, 0.5, 0.0),
            Vector3::new(0.5, -0.5, 0.0),
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(-0.5, 0.5, 0.0),
        ]
        .into_iter()
        .flatten()
        .collect(),
        3,
    );
    let colors = Buffer::new(
        vec![
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 1.0, 1.0),
        ]
        .into_iter()
        .flatten()
        .collect(),
        3,
    );

    let indices: Vec<u32> = vec![Vector3::new(0, 1, 3), Vector3::new(1, 2, 3)]
        .into_iter()
        .flatten()
        .collect();

    // let mesh = Mesh::from_buffers(vec![vertices], indices).unwrap();
    let color_mesh = Mesh::from_buffers(vec![vertices, colors], indices).unwrap();

    let vertex_shader = Shader::from_source(
        include_str!("shaders/triangle_color.vert"),
        ShaderType::VertexShader,
    )
    .unwrap();

    let fragment_shader = Shader::from_source(
        include_str!("shaders/triangle_color.frag"),
        ShaderType::FragmentShader,
    )
    .unwrap();

    let shader_program = Program::from_shaders(&vertex_shader, &fragment_shader, None).unwrap();

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.use_program();
        /*shader_program.set_value(
            &CString::new("colorScale").unwrap(),
            ProgramValue::Float(((glfw.get_time().sin() / 2.0) + 0.5) as f32),
        );*/
        color_mesh.draw();

        window.swap_buffers();
        glfw.poll_events();
    }
}
