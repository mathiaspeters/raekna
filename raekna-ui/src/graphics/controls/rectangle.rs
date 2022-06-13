use crate::graphics::vertex::Vertex;

#[derive(Copy, Clone, Debug, Default)]
pub struct Rectangle {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
    pub color: [f32; 3],
}

impl Rectangle {
    pub fn as_vertices(&self) -> [Vertex; 4] {
        [
            Vertex {
                position: [self.left, self.top, 1.0],
                color: self.color,
            },
            Vertex {
                position: [self.right, self.top, 1.0],
                color: self.color,
            },
            Vertex {
                position: [self.right, self.bottom, 1.0],
                color: self.color,
            },
            Vertex {
                position: [self.left, self.bottom, 1.0],
                color: self.color,
            },
        ]
    }
}
