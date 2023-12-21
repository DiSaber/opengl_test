#[derive(Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Default,
{
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> IntoIterator for Vector2<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y].into_iter()
    }
}
