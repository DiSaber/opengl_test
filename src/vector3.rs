#[derive(Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Default,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }

    pub fn default() -> Self {
        Vector3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> IntoIterator for Vector3<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}
