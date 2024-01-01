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
                -self.position
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
            rotation * position
        } else {
            position * rotation * scale
        }
    }

    pub fn set_euler_angles(&mut self, euler_angles: &glm::Vec3) {
        self.rotation = glm::quat_angle_axis(euler_angles.y, &glm::vec3(0.0, 1.0, 0.0))
            * glm::quat_angle_axis(euler_angles.x, &glm::vec3(1.0, 0.0, 0.0))
            * glm::quat_angle_axis(euler_angles.z, &glm::vec3(0.0, 0.0, 1.0));
    }

    pub fn set_euler_angles_deg(&mut self, euler_angles: &glm::Vec3) {
        self.set_euler_angles(&glm::vec3(
            euler_angles.x.to_radians(),
            euler_angles.y.to_radians(),
            euler_angles.z.to_radians(),
        ));
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
