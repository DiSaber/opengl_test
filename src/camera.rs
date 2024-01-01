use crate::{mesh_object::MeshObject, transform::Transform};

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub near_clipping_plane: f32,
    pub far_clipping_plane: f32,
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
            fov,
            screen_width,
            screen_height,
            near_clipping_plane,
            far_clipping_plane,
        }
    }

    pub fn draw_objects(&self, objects: &[&MeshObject]) {
        for object in objects {
            object.draw(self);
        }
    }

    pub fn get_projection_matrix(&self) -> na::Matrix4<f32> {
        na::geometry::Perspective3::new(
            (self.screen_width as f32) / (self.screen_height as f32),
            self.fov.to_radians(),
            self.near_clipping_plane,
            self.far_clipping_plane,
        )
        .into()
    }

    pub fn get_transform_matrix(&self) -> na::Matrix4<f32> {
        self.transform.to_matrix(true)
    }
}
