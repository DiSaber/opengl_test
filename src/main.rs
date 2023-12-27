extern crate gl;
extern crate glfw;
extern crate nalgebra_glm as glm;

mod camera;
mod mesh;
mod mesh_object;
mod program;
mod shader;
mod texture;
mod transform;
mod utils;
mod vertex;

use camera::Camera;
use glfw::Context;
use mesh::Mesh;
use mesh_object::MeshObject;
use program::Program;
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

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut main_camera = Camera::new(glm::perspective_fov(
        90f32.to_radians(),
        window.get_framebuffer_size().0 as f32,
        window.get_framebuffer_size().1 as f32,
        0.3,
        100.0,
    ));
    main_camera.transform.position = glm::vec3(0.0, 0.0, 1.0);

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

    let texture = Texture::from_image_bytes(
        include_bytes!("textures/container.jpg"),
        image::ImageFormat::Jpeg,
    );

    let mut mesh_object = MeshObject::new(&mesh, &[&texture], &shader_program);

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        mesh_object.transform = Default::default();
        mesh_object.transform.rotation = glm::quat_rotate(
            &mesh_object.transform.rotation,
            glfw.get_time() as f32,
            &glm::vec3(0.0, 0.0, 1.0),
        );
        mesh_object.transform.position = glm::vec3(0.5, 0.0, 0.0);
        mesh_object.transform.scale = glm::vec3(0.5, 0.5, 0.5);
        main_camera.draw_objects(&[&mesh_object]);

        window.swap_buffers();
        glfw.poll_events();
    }
}
