use crate::{vector2::Vector2, vector3::Vector3};

pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>) -> Self {
        Vertex {
            position,
            normal: Vector3::default(),
            tex_coord: Vector2::default(),
        }
    }

    pub fn vec_size() -> usize {
        Self::lengths().iter().sum::<usize>() * std::mem::size_of::<f32>()
    }

    pub fn lengths() -> Vec<usize> {
        vec![3, 3, 2]
    }
}

impl IntoIterator for Vertex {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.position.x,
            self.position.y,
            self.position.z,
            self.normal.x,
            self.normal.y,
            self.normal.z,
            self.tex_coord.x,
            self.tex_coord.y,
        ]
        .into_iter()
    }
}
