mod vector3;

use std::ffi::CString;

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

    let (mut window, _events) = glfw
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

    let mut vbo = 0u32;
    let vertex_shader_source = CString::new(include_str!("shaders/triangle.vert")).unwrap();
    let fragment_shader_source = CString::new(include_str!("shaders/triangle.frag")).unwrap();

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (triangle_vertices.len() * std::mem::size_of::<f32>()) as isize,
            triangle_vertices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        let mut success = 0i32;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut error_length = 0i32;
            gl::GetShaderiv(vertex_shader, gl::INFO_LOG_LENGTH, &mut error_length);

            let error = CString::from_vec_unchecked(
                [b' ']
                    .iter()
                    .cycle()
                    .take(error_length as usize)
                    .cloned()
                    .collect::<Vec<u8>>(),
            );
            gl::GetShaderInfoLog(
                vertex_shader,
                error_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut i8,
            )
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        let mut success = 0i32;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut error_length = 0i32;
            gl::GetShaderiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut error_length);

            let error = CString::from_vec_unchecked(
                [b' ']
                    .iter()
                    .cycle()
                    .take(error_length as usize)
                    .cloned()
                    .collect::<Vec<u8>>(),
            );
            gl::GetShaderInfoLog(
                fragment_shader,
                error_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut i8,
            )
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0i32;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut error_length = 0i32;
            gl::GetProgramiv(fragment_shader, gl::INFO_LOG_LENGTH, &mut error_length);

            let error = CString::from_vec_unchecked(
                [b' ']
                    .iter()
                    .cycle()
                    .take(error_length as usize)
                    .cloned()
                    .collect::<Vec<u8>>(),
            );
            gl::GetProgramInfoLog(
                fragment_shader,
                error_length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut i8,
            )
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
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
