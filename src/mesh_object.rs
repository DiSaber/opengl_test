use crate::{
    mesh::Mesh,
    program::{Program, ProgramValue},
    texture::Texture,
    transform::Transform,
};

pub struct MeshObject<'a> {
    mesh: &'a Mesh,
    textures: Vec<&'a Texture>,
    shader_program: &'a Program,
    pub transform: Transform,
}

impl<'a> MeshObject<'a> {
    pub fn new(mesh: &'a Mesh, textures: &[&'a Texture], shader_program: &'a Program) -> Self {
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

        mesh_object.set_shader_value(
            "transform",
            ProgramValue::Mat4(mesh_object.transform.to_matrix()),
        );

        mesh_object
    }

    pub fn draw(&self) {
        self.set_shader_value("transform", ProgramValue::Mat4(self.transform.to_matrix()));
        self.mesh.draw(&self.textures);
    }

    pub fn set_shader_value(&self, name: &str, value: ProgramValue) {
        self.shader_program.set_value(name, value)
    }
}