extern crate nalgebra as na;

mod camera;
mod mesh;
mod mesh_object;
mod shader;
mod shader_program;
mod texture;
mod transform;
mod utils;
mod vertex;

use std::{collections::HashMap, fs};

use camera::Camera;
use glfw::Context;
use mesh::Mesh;
use mesh_object::MeshObject;
use na::{Vector2, Vector3};
use shader::{Shader, ShaderType};
use shader_program::ShaderProgram;
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

    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let resources: HashMap<String, Vec<u8>> =
        bincode::deserialize(&fs::read(exe_dir.join("resources.pck")).unwrap()).unwrap();

    let models = utils::tobj_from_slice_no_mtl(resources["slope.obj"].as_slice()).unwrap();

    let slope_mesh = Mesh::from_tobj(&models[0].mesh);

    let vertices = vec![
        Vertex::tex(Vector3::new(0.5, 0.5, 0.0), Vector2::new(1.0, 0.0)),
        Vertex::tex(Vector3::new(0.5, -0.5, 0.0), Vector2::new(1.0, 1.0)),
        Vertex::tex(Vector3::new(-0.5, -0.5, 0.0), Vector2::new(0.0, 1.0)),
        Vertex::tex(Vector3::new(-0.5, 0.5, 0.0), Vector2::new(0.0, 0.0)),
    ];
    let faces = vec![Vector3::new(0, 1, 3), Vector3::new(1, 2, 3)];

    let mesh = Mesh::from_vertices(vertices, faces);

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
        0.01,
        100.0,
        palette::LinSrgba::new(0.2, 0.3, 0.3, 1.0),
    );
    main_camera.transform.position = Vector3::new(0.0, 0.0, 1.0);

    let mut mesh_object = MeshObject::new(&mesh, &[&texture], &shader_program);
    mesh_object.transform.position = Vector3::new(-1.0, -0.5, 0.0);
    let mut mesh_object2 = MeshObject::new(&mesh, &[&texture], &shader_program);
    mesh_object2.transform.position = Vector3::new(0.0, -0.5, 2.0);

    let mut slope_object = MeshObject::new(&slope_mesh, &[&texture], &shader_program);
    slope_object.transform.position = Vector3::new(0.0, 0.0, -1.0);
    slope_object
        .transform
        .set_euler_angles_deg(&Vector3::new(0.0, 135.0_f32, 0.0));
    let mut floor_object = MeshObject::new(&mesh, &[&texture], &shader_program);
    floor_object
        .transform
        .set_euler_angles_deg(&Vector3::new(90.0_f32, 0.0, 0.0));
    floor_object.transform.position = Vector3::new(0.0, -0.5, 0.0);
    floor_object.transform.scale = Vector3::new(5.0, 5.0, 5.0);

    let mut last_frame_time = glfw.get_time();

    window.set_cursor_mode(glfw::CursorMode::Disabled);
    let mouse_sensitivity = 0.1;
    let movement_sensitivity = 1.0;
    let (mut last_mouse_x, mut last_mouse_y) = window.get_cursor_pos();
    let mut camera_rotation = Vector3::<f32>::zeros();

    while !window.should_close() {
        if window.get_key(glfw::Key::Escape) == glfw::Action::Press {
            window.set_should_close(true);
        }

        let (mouse_x, mouse_y) = window.get_cursor_pos();
        camera_rotation = Vector3::new(
            (camera_rotation.x - (((mouse_y - last_mouse_y) * mouse_sensitivity) as f32))
                .clamp(-90.0, 90.0),
            camera_rotation.y - (((mouse_x - last_mouse_x) * mouse_sensitivity) as f32),
            0.0,
        );
        main_camera.transform.set_euler_angles_deg(&camera_rotation);

        let current_time = glfw.get_time();
        let delta_time = current_time - last_frame_time;

        let mut movement = Vector3::<f32>::zeros();
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            movement.z -= 1.0;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            movement.z += 1.0;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            movement.x += 1.0;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            movement.x -= 1.0;
        }
        if window.get_key(glfw::Key::E) == glfw::Action::Press {
            movement.y += 1.0;
        }
        if window.get_key(glfw::Key::Q) == glfw::Action::Press {
            movement.y -= 1.0;
        }
        main_camera.transform.position += main_camera.transform.rotation
            * movement
            * (delta_time as f32)
            * (if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
                movement_sensitivity * 2.0
            } else {
                movement_sensitivity
            } as f32);

        mesh_object
            .transform
            .set_euler_angles(&Vector3::new(0.0, 0.0, glfw.get_time() as f32));
        mesh_object2
            .transform
            .set_euler_angles(&Vector3::new(0.0, 0.0, glfw.get_time() as f32));

        let (width, height) = window.get_framebuffer_size();
        main_camera.set_screen_size(width, height);
        main_camera.clear();
        main_camera.draw_objects(&[&mesh_object, &floor_object, &mesh_object2, &slope_object]);

        (last_mouse_x, last_mouse_y) = (mouse_x, mouse_y);
        last_frame_time = current_time;

        window.swap_buffers();
        glfw.poll_events();
    }
}
