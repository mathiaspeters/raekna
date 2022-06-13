use super::{rectangle::Rectangle, update_indices, update_vertices};
use crate::{
    constants::{SEPARATOR_COLOR, SEPARATOR_WIDTH},
    coordinator::dimensions::Dimensions,
    graphics::vertex::Vertex,
};

#[derive(Default)]
pub struct Separator {
    rect: Rectangle,
}

impl Separator {
    pub fn new(
        dimensions: &Dimensions,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) -> Self {
        let rect = Rectangle {
            top: 1.0,
            bottom: -1.0,
            left: 0.0,
            right: 0.0,
            color: SEPARATOR_COLOR,
        };
        let mut separator = Self { rect };
        separator.update(dimensions, vertices);
        update_indices(indices, 1);
        separator
    }

    pub fn update(&mut self, dimensions: &Dimensions, vertices: &mut Vec<Vertex>) {
        let left_px = dimensions.separator_x_offset;
        let right_px = left_px + SEPARATOR_WIDTH;
        self.rect.left = dimensions.as_x_vertex(left_px);
        self.rect.right = dimensions.as_x_vertex(right_px);
        update_vertices(self.rect, vertices, 1);
    }
}
