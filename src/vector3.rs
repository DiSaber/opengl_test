pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FlattenVector3 for Vector3 {
    fn flatten(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

impl FlattenVector3 for Vec<Vector3> {
    fn flatten(&self) -> Vec<f32> {
        self.iter().flat_map(|vector| vector.flatten()).collect()
    }
}

pub trait FlattenVector3 {
    fn flatten(&self) -> Vec<f32>;
}
