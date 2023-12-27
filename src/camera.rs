use crate::{mesh_object::MeshObject, transform::Transform};

pub struct Camera {
    pub transform: Transform,
    pub projection: glm::Mat4,
}

impl Camera {
    pub fn new(projection: glm::Mat4) -> Camera {
        Camera {
            transform: Default::default(),
            projection,
        }
    }

    pub fn draw_objects(&self, objects: &[&MeshObject]) {
        for object in objects {
            object.draw(self);
        }
    }
}
