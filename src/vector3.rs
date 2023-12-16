use crate::utils::FlattenToVec;

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FlattenToVec<f32> for Vector3 {
    fn flatten(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}
