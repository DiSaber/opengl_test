#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
}

impl Transform {
    pub fn to_matrix(&self, is_camera: bool) -> glm::Mat4 {
        let position = glm::translate(
            &glm::identity(),
            &if is_camera {
                glm::vec3(-self.position.x, -self.position.y, self.position.z)
            } else {
                self.position
            },
        );
        let rotation = glm::quat_to_mat4(&if is_camera {
            glm::quat_inverse(&self.rotation)
        } else {
            self.rotation
        });
        let scale = glm::scale(&glm::identity(), &self.scale);
        if is_camera {
            rotation * position * scale
        } else {
            position * rotation * scale
        }
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
