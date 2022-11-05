use std::time::Instant;

use super::{
    caret_position::CaretPosition, clear_indices, rectangle::Rectangle, update_indices,
    update_vertices,
};
use crate::{
    constants::{CARET_COLOR, CARET_PERIOD, CARET_WIDTH, TEXT_PADDING},
    coordinator::dimensions::Dimensions,
    rendering::vertex::Vertex,
};

pub struct Caret {
    last_visibility_update: Instant,
    should_show: bool,
    line: usize,
    column: usize,
    rect: Rectangle,
}

impl Caret {
    pub fn new(
        dimensions: &Dimensions,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
        caret_position: &CaretPosition,
    ) -> Self {
        let last_visibility_update = Instant::now();
        let should_show = true;
        let line = caret_position.line;
        let column = caret_position.column;
        let rect = Rectangle {
            color: CARET_COLOR,
            ..Default::default()
        };
        let mut caret = Self {
            last_visibility_update,
            should_show,
            line,
            column,
            rect,
        };
        caret.update(dimensions, vertices);
        update_indices(indices, 0);
        caret
    }

    pub fn update_time(&mut self, indices: &mut Vec<u16>) -> (bool, Option<Instant>) {
        let elapsed = self.last_visibility_update.elapsed().as_millis();
        let should_update_visibility = elapsed >= CARET_PERIOD as u128;
        if should_update_visibility {
            self.last_visibility_update = Instant::now();
            self.should_show = !self.should_show;
            if self.should_show {
                update_indices(indices, 0);
            } else {
                clear_indices(indices, 0)
            }
        }
        let last_visibility_update = if self.rect.bottom <= 1.0 && self.rect.top >= -1.0 {
            Some(self.last_visibility_update)
        } else {
            None
        };
        (should_update_visibility, last_visibility_update)
    }

    pub fn update_position(&mut self, caret_position: &CaretPosition, indices: &mut Vec<u16>) {
        self.line = caret_position.line;
        self.column = caret_position.column;
        self.set_visible(indices);
    }

    pub fn update(&mut self, dimensions: &Dimensions, vertices: &mut Vec<Vertex>) {
        let (top_px, bottom_px) = self.vertical_px(dimensions);
        let left_px = TEXT_PADDING + (self.column as f32 * dimensions.glyph_width());
        let right_px = left_px + CARET_WIDTH;

        self.rect.top = dimensions.as_y_vertex(top_px);
        self.rect.bottom = dimensions.as_y_vertex(bottom_px);
        self.rect.left = dimensions.as_x_vertex(left_px);
        self.rect.right = dimensions.as_x_vertex(right_px);

        update_vertices(self.rect, vertices, 0);
    }

    pub fn set_visible(&mut self, indices: &mut Vec<u16>) {
        self.last_visibility_update = Instant::now();
        self.should_show = true;
        update_indices(indices, 0);
    }

    pub fn vertical_px(&self, dimensions: &Dimensions) -> (f32, f32) {
        let top_px = TEXT_PADDING - dimensions.scroll()
            + (self.line as f32 * (dimensions.glyph_height() + TEXT_PADDING))
            + (dimensions.glyph_height() * 0.05);
        let bottom_px = top_px + (dimensions.glyph_height() * 0.9);
        (top_px, bottom_px)
    }
}
