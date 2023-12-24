use glm::{Vec2, Vec3};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coord: Vec2,
}

impl Vertex {
    pub fn tex(position: Vec3, tex_coord: Vec2) -> Self {
        Vertex {
            position,
            normal: Vec3::zeros(),
            tex_coord,
        }
    }

    pub fn c_size() -> usize {
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
            glm::value_ptr(&self.position),
            glm::value_ptr(&self.normal),
            glm::value_ptr(&self.tex_coord),
        ]
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<f32>>()
        .into_iter()
    }
}
