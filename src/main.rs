use std::{collections::HashMap, fs};

use my_gl::{
    image::{self, ImageFormat},
    na::{Vector2, Vector3},
    palette::LinSrgba,
    utils, Action, Camera, FilterMode, Font, Game, Key, Mesh, MeshObject, MouseMode,
    OrthographicType, Shader, ShaderProgram, ShaderType, TextObject, Texture, UsageType, Vertex,
    WrapMode,
};

fn main() {
    let mut game = Game::new(800, 600, "OpenGL Test", my_gl::WindowMode::Windowed);

    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let resources: HashMap<String, Vec<u8>> =
        bincode::deserialize(&fs::read(exe_dir.join("resources.pck")).unwrap()).unwrap();

    let models = utils::tobj_from_slice_no_mtl(resources["slope.obj"].as_slice()).unwrap();

    let slope_mesh = Mesh::from_tobj(&models[0].mesh, UsageType::Static);

    let vertices = vec![
        Vertex::tex(Vector3::new(0.5, 0.5, 0.0), Vector2::new(1.0, 0.0)),
        Vertex::tex(Vector3::new(0.5, -0.5, 0.0), Vector2::new(1.0, 1.0)),
        Vertex::tex(Vector3::new(-0.5, -0.5, 0.0), Vector2::new(0.0, 1.0)),
        Vertex::tex(Vector3::new(-0.5, 0.5, 0.0), Vector2::new(0.0, 0.0)),
    ];
    let faces = vec![Vector3::new(0, 1, 3), Vector3::new(1, 2, 3)];

    let mesh = Mesh::from_vertices(&vertices, &faces, UsageType::Static);
    let mesh2 = mesh.clone();

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

    let texture = Texture::from_image(
        image::load_from_memory_with_format(&resources["container.jpg"], ImageFormat::Jpeg)
            .unwrap(),
        WrapMode::Repeat,
        FilterMode::LinearMipMapLinear,
        FilterMode::Linear,
    );

    let mut main_camera = Camera::new_perspective(
        90.0,
        game.get_framebuffer_size(),
        0.01,
        100.0,
        LinSrgba::new(0.2, 0.3, 0.3, 1.0),
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
    let mut floor_object = MeshObject::new(&mesh2, &[&texture], &shader_program);
    floor_object
        .transform
        .set_euler_angles_deg(&Vector3::new(90.0_f32, 0.0, 0.0));
    floor_object.transform.position = Vector3::new(0.0, -0.5, 0.0);
    floor_object.transform.scale = Vector3::new(5.0, 5.0, 5.0);

    let mut ui_camera = Camera::new_orthographic(
        OrthographicType::UI { height: 600.0 },
        game.get_framebuffer_size(),
        0.1,
        100.0,
        LinSrgba::new(0.0, 0.0, 0.0, 0.0),
    );
    ui_camera.transform.position = Vector3::new(0.0, 0.0, 1.0);

    let ui_fragment_shader =
        Shader::from_source(&resources["ui_text.frag"], ShaderType::FragmentShader).unwrap();
    let ui_shader_program =
        ShaderProgram::from_shaders(&vertex_shader, &ui_fragment_shader).unwrap();

    let font = Font::from_bytes(&resources["Arial.ttf"]).unwrap();
    let mut text_object = TextObject::new(
        "".into(),
        32,
        LinSrgba::new(1.0, 0.2, 0.2, 1.0),
        &ui_shader_program,
        &font,
        UsageType::Dynamic,
    );

    game.set_mouse_mode(MouseMode::Disabled);
    let mouse_sensitivity = 0.1;
    let movement_sensitivity = 1.0;
    let (mut last_mouse_x, mut last_mouse_y) = game.get_mouse_position();
    let mut camera_rotation = Vector3::<f32>::zeros();

    game.run_update(|game, delta_time| {
        if game.get_key(Key::Escape) == Action::Press {
            game.close_window();
        }

        let (mouse_x, mouse_y) = game.get_mouse_position();
        camera_rotation = Vector3::new(
            (camera_rotation.x - (((mouse_y - last_mouse_y) * mouse_sensitivity) as f32))
                .clamp(-90.0, 90.0),
            camera_rotation.y - (((mouse_x - last_mouse_x) * mouse_sensitivity) as f32),
            0.0,
        );
        main_camera.transform.set_euler_angles_deg(&camera_rotation);

        let mut movement = Vector3::<f32>::zeros();
        if game.get_key(Key::W) == Action::Press {
            movement.z -= 1.0;
        }
        if game.get_key(Key::S) == Action::Press {
            movement.z += 1.0;
        }
        if game.get_key(Key::D) == Action::Press {
            movement.x += 1.0;
        }
        if game.get_key(Key::A) == Action::Press {
            movement.x -= 1.0;
        }
        if game.get_key(Key::E) == Action::Press {
            movement.y += 1.0;
        }
        if game.get_key(Key::Q) == Action::Press {
            movement.y -= 1.0;
        }
        main_camera.transform.position += main_camera.transform.rotation
            * movement
            * (delta_time as f32)
            * (if game.get_key(Key::LeftShift) == Action::Press {
                movement_sensitivity * 2.0
            } else {
                movement_sensitivity
            } as f32);

        mesh_object
            .transform
            .set_euler_angles(&Vector3::new(0.0, 0.0, game.get_time() as f32));
        mesh_object2
            .transform
            .set_euler_angles(&Vector3::new(0.0, 0.0, game.get_time() as f32));

        main_camera.set_screen_size(game.get_framebuffer_size());
        main_camera.clear();
        main_camera.draw_objects(&mut [
            &mut mesh_object,
            &mut floor_object,
            &mut mesh_object2,
            &mut slope_object,
        ]);

        text_object.text = format!("FPS: {:.0}\nHello World", 1.0 / delta_time);

        ui_camera.set_screen_size(game.get_framebuffer_size());
        ui_camera.draw_objects(&mut [&mut text_object]);

        (last_mouse_x, last_mouse_y) = (mouse_x, mouse_y);
    });
}
