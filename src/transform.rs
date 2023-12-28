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
            glm::vec3(-self.position.x, -self.position.y, self.position.z)
        } else {
            self.position
        };
        transform = glm::translate(&transform, &position);
        let normalized_rotation = if invert_position {
            glm::quat_normalize(&glm::quat_inverse(&self.rotation))
        } else {
            glm::quat_normalize(&self.rotation)
        };
        transform = glm::rotate(
            &transform,
            glm::quat_angle(&normalized_rotation),
            &glm::quat_axis(&normalized_rotation),
        );
        transform = glm::scale(&transform, &self.scale);
        let position = glm::translate(&glm::identity(), &self.position);
        let rotation = glm::quat_to_mat4(&self.rotation);
        let scale = glm::scale(&glm::identity(), &self.scale);
        //let translated_position = glm::quat_rotate_vec3(&normalized_rotation, &position);
        //let position = glm::translate(&glm::identity(), &translated_position);
        //position * rotation * scale
        position * rotation * scale
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
