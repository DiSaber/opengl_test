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

impl<T, U> FlattenToVec<T> for Vec<U>
where
    U: FlattenToVec<T>,
{
    fn flatten(&self) -> Vec<T> {
        self.iter().flat_map(|vector| vector.flatten()).collect()
    }
}

pub trait FlattenToVec<T> {
    fn flatten(&self) -> Vec<T>;
}
