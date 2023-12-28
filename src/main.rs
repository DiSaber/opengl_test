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

use std::{collections::HashMap, fs};

use camera::Camera;
use glfw::Context;
use mesh::Mesh;
use mesh_object::MeshObject;
use program::ShaderProgram;
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

    window.set_framebuffer_size_callback(|_, width, height| unsafe {
        gl::Viewport(0, 0, width, height);
    });

    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let resources: HashMap<String, Vec<u8>> =
        bincode::deserialize(&fs::read(exe_dir.join("resources.pck")).unwrap()).unwrap();

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
        &resources["triangle_texture.vert"],
        ShaderType::VertexShader,
    )
    .unwrap();

    let fragment_shader = Shader::from_source(
        &resources["triangle_texture.frag"],
        ShaderType::FragmentShader,
    )
    .unwrap();

    let shader_program = ShaderProgram::from_shaders(&vertex_shader, &fragment_shader).unwrap();

    let texture = Texture::from_image_bytes(&resources["container.jpg"], image::ImageFormat::Jpeg);

    let mut main_camera = Camera::new(
        90.0,
        window.get_framebuffer_size().0,
        window.get_framebuffer_size().1,
        0.3,
        100.0,
    );
    main_camera.transform.position = glm::vec3(0.0, 0.0, 1.0);

    let mut mesh_object = MeshObject::new(&mesh, &[&texture], &shader_program);
    mesh_object.transform.position = glm::vec3(0.0, -0.5, 0.0);
    let mut mesh_object2 = MeshObject::new(&mesh, &[&texture], &shader_program);
    mesh_object2.transform.position = glm::vec3(0.0, -0.5, 2.0);

    let mut floor_object = MeshObject::new(&mesh, &[&texture], &shader_program);
    floor_object.transform.rotation = glm::quat_rotate(
        &glm::quat_identity(),
        90.0_f32.to_radians(),
        &glm::vec3(1.0, 0.0, 0.0),
    );
    floor_object.transform.position = glm::vec3(0.0, -0.5, 0.0);
    floor_object.transform.scale = glm::vec3(5.0, 5.0, 5.0);

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        main_camera.screen_width = window.get_framebuffer_size().0;
        main_camera.screen_height = window.get_framebuffer_size().1;
        main_camera.transform.rotation = glm::quat_rotate(
            &glm::quat_identity(),
            glfw.get_time() as f32,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        mesh_object.transform.rotation = glm::quat_rotate(
            &glm::quat_identity(),
            glfw.get_time() as f32,
            &glm::vec3(0.0, 0.0, 1.0),
        );
        mesh_object2.transform.rotation = glm::quat_rotate(
            &glm::quat_identity(),
            glfw.get_time() as f32,
            &glm::vec3(0.0, 0.0, 1.0),
        );
        main_camera.draw_objects(&[&mesh_object, &floor_object, &mesh_object2]);

        window.swap_buffers();
        glfw.poll_events();
    }
}
