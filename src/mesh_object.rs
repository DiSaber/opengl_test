use crate::{
    camera::Camera,
    mesh::Mesh,
    program::{ProgramValue, ShaderProgram},
    texture::Texture,
    transform::Transform,
};

pub struct MeshObject<'a> {
    mesh: &'a Mesh,
    textures: Vec<&'a Texture>,
    shader_program: &'a ShaderProgram,
    pub transform: Transform,
}

impl<'a> MeshObject<'a> {
    pub fn new(
        mesh: &'a Mesh,
        textures: &[&'a Texture],
        shader_program: &'a ShaderProgram,
    ) -> Self {
        let mesh_object = MeshObject {
            mesh,
            textures: Vec::from(textures),
            shader_program,
            transform: Default::default(),
        };

        for (i, _) in mesh_object.textures.iter().enumerate() {
            mesh_object.set_shader_value(
                &("texture".to_owned() + &i.to_string()),
                ProgramValue::Int(i as i32),
            )
        }

        mesh_object
    }

    pub fn draw(&self, camera: &Camera) {
        self.set_shader_value("transform", ProgramValue::Mat4(self.get_transform_matrix()));
        self.set_shader_value(
            "camera_transform",
            ProgramValue::Mat4(camera.get_transform_matrix()),
        );
        self.set_shader_value(
            "camera_projection",
            ProgramValue::Mat4(camera.get_projection_matrix()),
        );
        self.mesh.draw(&self.textures);
    }

    pub fn set_shader_value(&self, name: &str, value: ProgramValue) {
        self.shader_program.set_value(name, value)
    }

    pub fn get_transform_matrix(&self) -> glm::Mat4 {
        self.transform.to_matrix(false)
    }
}
