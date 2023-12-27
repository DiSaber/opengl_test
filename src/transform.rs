#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
}

impl Transform {
    pub fn to_matrix(&self, invert_position: bool) -> glm::Mat4 {
        let mut transform: glm::Mat4 = glm::identity();
        let position = if invert_position {
            -self.position
        } else {
            self.position
        };
        transform = glm::translate(&transform, &position);
        let normalized_rotation = glm::quat_normalize(&self.rotation);
        transform = glm::rotate(
            &transform,
            glm::quat_angle(&normalized_rotation),
            &glm::quat_axis(&normalized_rotation),
        );
        transform = glm::scale(&transform, &self.scale);

        transform
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: glm::quat_identity(),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }
}
