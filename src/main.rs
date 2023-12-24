extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod mesh;
mod program;
mod shader;
mod texture;
mod utils;
mod vertex;

use glfw::Context;
use mesh::Mesh;
use program::{Program, ProgramValue};
use shader::{Shader, ShaderType};
use texture::Texture;
use vertex::Vertex;

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

    let vertices = vec![
        Vertex::tex(glm::vec3(0.5, 0.5, 0.0), glm::vec2(1.0, 0.0)),
        Vertex::tex(glm::vec3(0.5, -0.5, 0.0), glm::vec2(1.0, 1.0)),
        Vertex::tex(glm::vec3(-0.5, -0.5, 0.0), glm::vec2(0.0, 1.0)),
        Vertex::tex(glm::vec3(-0.5, 0.5, 0.0), glm::vec2(0.0, 0.0)),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<f32>>();

    let indices = vec![glm::vec3(0, 1, 3), glm::vec3(1, 2, 3)]
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u32>>();

    let mesh = Mesh::from_buffer(&vertices, &indices);

    let vertex_shader = Shader::from_source(
        include_bytes!("shaders/triangle_texture.vert"),
        ShaderType::VertexShader,
    )
    .unwrap();

    let fragment_shader = Shader::from_source(
        include_bytes!("shaders/triangle_texture.frag"),
        ShaderType::FragmentShader,
    )
    .unwrap();

    let shader_program = Program::from_shaders(&vertex_shader, &fragment_shader).unwrap();
    shader_program.set_value("texture1", ProgramValue::Int(0));

    let texture = Texture::from_image_bytes(
        include_bytes!("textures/container.jpg"),
        image::ImageFormat::Jpeg,
    );

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
            "colorScale",
            ProgramValue::Float(((glfw.get_time().sin() / 2.0) + 0.5) as f32),
        );*/
        mesh.draw(&vec![&texture]);

        window.swap_buffers();
        glfw.poll_events();
    }
}
