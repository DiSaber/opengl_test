use na::{Vector2, Vector3};

pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
}

impl Vertex {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Vertex {
            position,
            normal,
            tex_coord,
        }
    }

    pub fn tex(position: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Vertex {
            position,
            normal: Vector3::zeros(),
            tex_coord,
        }
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
            self.position.as_slice(),
            self.normal.as_slice(),
            self.tex_coord.as_slice(),
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<f32>>()
        .into_iter()
    }
}
