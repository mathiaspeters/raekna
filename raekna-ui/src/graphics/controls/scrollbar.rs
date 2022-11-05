use super::{rectangle::Rectangle, update_indices, update_vertices, ScrollHandleState};
use crate::{
    constants::{SCROLLBAR_BACKGROUND_COLOR, SCROLLBAR_BASE_COLOR, SCROLLBAR_WIDTH_MULTIPLIER},
    coordinator::dimensions::Dimensions,
    rendering::vertex::Vertex,
};

pub struct Scrollbar {
    bg_rect: Rectangle,
    pub handle_rect: Rectangle,
    pub handle_state: ScrollHandleState,
}

impl Scrollbar {
    pub fn new(
        dimensions: &Dimensions,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) -> Self {
        let bg_rect = Rectangle {
            top: 1.0,
            bottom: -1.0,
            left: 0.0,
            right: 1.0,
            color: SCROLLBAR_BACKGROUND_COLOR,
        };
        let mut handle_rect = bg_rect;
        handle_rect.color = SCROLLBAR_BASE_COLOR;
        let mut scrollbar = Self {
            bg_rect,
            handle_rect,
            handle_state: ScrollHandleState::Default,
        };
        scrollbar.update(dimensions, vertices, indices);
        update_indices(indices, 2);
        scrollbar
    }

    pub fn update(
        &mut self,
        dimensions: &Dimensions,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) {
        let left_px =
            dimensions.window_width() - (dimensions.glyph_width() * SCROLLBAR_WIDTH_MULTIPLIER);
        let left = dimensions.as_x_vertex(left_px);
        self.bg_rect.left = left;
        update_vertices(self.bg_rect, vertices, 2);
        if dimensions.scroll_ratio >= 1.0 {
            self.handle_rect.left = self.handle_rect.right;
            self.handle_rect.top = 1.0;
            self.handle_rect.bottom = 1.0;
        } else {
            let window_height = dimensions.window_height();
            let space_needed = window_height / dimensions.scroll_ratio;
            let top_px = (dimensions.scroll() / space_needed) * window_height;
            let bottom_px = top_px + window_height * dimensions.scroll_ratio;
            let top = dimensions.as_y_vertex(top_px);
            let bottom = dimensions.as_y_vertex(bottom_px);
            self.handle_rect.left = left;
            self.handle_rect.top = top;
            self.handle_rect.bottom = bottom;
        }
        update_vertices(self.handle_rect, vertices, 3);
        update_indices(indices, 3)
    }
}
