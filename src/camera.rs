use crate::{mesh_object::MeshObject, transform::Transform};

pub struct Camera {
    pub transform: Transform,
    perspective: na::geometry::Perspective3<f32>,
}

impl Camera {
    pub fn new(
        fov: f32,
        screen_width: i32,
        screen_height: i32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
    ) -> Camera {
        Camera {
            transform: Default::default(),
            perspective: na::geometry::Perspective3::new(
                (screen_width as f32) / (screen_height as f32),
                fov.to_radians(),
                near_clipping_plane,
                far_clipping_plane,
            ),
        }
    }

    pub fn draw_objects(&self, objects: &[&MeshObject]) {
        for object in objects {
            object.draw(self);
        }
    }

    pub fn get_projection_matrix(&self) -> na::Matrix4<f32> {
        self.perspective.into()
    }

    pub fn get_transform_matrix(&self) -> na::Matrix4<f32> {
        self.transform.to_matrix(true)
    }

    pub fn set_screen_size(&mut self, screen_width: i32, screen_height: i32) {
        self.perspective
            .set_aspect((screen_width as f32) / (screen_height as f32));
    }
}
