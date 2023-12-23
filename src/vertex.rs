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
