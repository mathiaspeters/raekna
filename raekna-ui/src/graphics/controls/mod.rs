use std::time::Instant;

pub use selection::get_ordered_selection;
use wgpu::Device;
use winit::dpi::PhysicalPosition;

use self::{
    caret::Caret, caret_position::CaretPosition, rectangle::Rectangle, scrollbar::Scrollbar,
    selection::Selection, separator::Separator,
};
use super::{buffers::Buffers, vertex::Vertex};
use crate::{
    constants::{SCROLLBAR_BASE_COLOR, SCROLLBAR_CLICK_COLOR, SCROLLBAR_HOVER_COLOR},
    coordinator::dimensions::Dimensions,
};

mod caret;
pub mod caret_position;
mod rectangle;
mod scrollbar;
pub mod selection;
mod separator;

const NUM_VERTICES: usize = 16;
const NUM_INDICES: usize = 24;

#[derive(Copy, Clone, Debug)]
pub enum ScrollHandleState {
    Default,
    Hover,
    Clicked,
}

pub struct Controls {
    pub caret: Caret,
    separator: Separator,
    pub scrollbar: Scrollbar,
    selection: Selection,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Controls {
    pub fn new(dimensions: &Dimensions, caret_position: &CaretPosition) -> Self {
        let mut vertices = vec![Default::default(); NUM_VERTICES];
        let mut indices = vec![0; NUM_INDICES];
        let caret = Caret::new(dimensions, &mut vertices, &mut indices, caret_position);
        let separator = Separator::new(dimensions, &mut vertices, &mut indices);
        let scrollbar = Scrollbar::new(dimensions, &mut vertices, &mut indices);
        let selection = Default::default();

        Self {
            caret,
            separator,
            scrollbar,
            selection,
            vertices,
            indices,
        }
    }

    pub fn update(&mut self, dimensions: &Dimensions, line_widths: &[usize]) {
        self.caret.update(dimensions, &mut self.vertices);
        self.separator.update(dimensions, &mut self.vertices);
        self.scrollbar
            .update(dimensions, &mut self.vertices, &mut self.indices);
        self.selection.update(
            dimensions,
            line_widths,
            &mut self.vertices,
            &mut self.indices,
        );
    }

    pub fn update_caret_position(
        &mut self,
        dimensions: &Dimensions,
        caret_position: &CaretPosition,
    ) {
        self.caret
            .update_position(caret_position, &mut self.indices);
        self.caret.update(dimensions, &mut self.vertices);
    }

    pub fn show_selection(
        &mut self,
        dimensions: &Dimensions,
        line_widths: &[usize],
        selection_start: CaretPosition,
        selection_end: CaretPosition,
    ) {
        self.selection.show_selection(
            dimensions,
            line_widths,
            selection_start,
            selection_end,
            &mut self.vertices,
            &mut self.indices,
        );
    }

    pub fn hide_selection(&mut self) {
        self.selection
            .hide_selection(&mut self.vertices, &mut self.indices);
    }

    pub fn update_time(&mut self) -> (bool, Option<Instant>) {
        self.caret.update_time(&mut self.indices)
    }

    pub fn get_as_buffers(&self, device: &Device) -> Buffers {
        Buffers::new(device, &self.vertices, &self.indices)
    }

    pub fn is_in_scroll_bar(
        &self,
        dimensions: &Dimensions,
        position: PhysicalPosition<f32>,
    ) -> bool {
        let mouse_x = dimensions.as_x_vertex(position.x);
        let rect = self.scrollbar.handle_rect;
        mouse_x > rect.left && mouse_x < rect.right
    }

    pub fn is_in_scroll_handle(
        &self,
        dimensions: &Dimensions,
        position: PhysicalPosition<f32>,
    ) -> bool {
        let mouse_x = dimensions.as_x_vertex(position.x);
        let mouse_y = dimensions.as_y_vertex(position.y);
        let rect = self.scrollbar.handle_rect;
        mouse_x > rect.left && mouse_x < rect.right && mouse_y < rect.top && mouse_y > rect.bottom
    }

    pub fn update_handle_state(&mut self, state: ScrollHandleState) {
        self.scrollbar.handle_state = state;
        match state {
            ScrollHandleState::Default => self.scrollbar.handle_rect.color = SCROLLBAR_BASE_COLOR,
            ScrollHandleState::Hover => self.scrollbar.handle_rect.color = SCROLLBAR_HOVER_COLOR,
            ScrollHandleState::Clicked => self.scrollbar.handle_rect.color = SCROLLBAR_CLICK_COLOR,
        }
    }

    pub fn set_caret_visible(&mut self) {
        self.caret.set_visible(&mut self.indices);
    }
}

fn update_vertices(rect: Rectangle, vertices: &mut Vec<Vertex>, offset: usize) {
    let write_offset = 4 * offset;
    if vertices.len() < 4 * (offset + 1) {
        vertices.resize(4 * (offset + 1), Default::default());
    }
    let rv = rect.as_vertices();
    vertices[write_offset] = rv[0];
    vertices[write_offset + 1] = rv[1];
    vertices[write_offset + 2] = rv[2];
    vertices[write_offset + 3] = rv[3];
}

fn update_indices(indices: &mut Vec<u16>, offset: usize) {
    let write_offset = 6 * offset;
    let vertex_offset = 4 * offset as u16;
    if indices.len() < 6 * (offset + 1) {
        indices.resize(6 * (offset + 1), 0);
    }
    indices[write_offset] = vertex_offset + 1;
    indices[write_offset + 1] = vertex_offset;
    indices[write_offset + 2] = vertex_offset + 3;
    indices[write_offset + 3] = vertex_offset + 1;
    indices[write_offset + 4] = vertex_offset + 3;
    indices[write_offset + 5] = vertex_offset + 2;
}

fn clear_indices(indices: &mut [u16], offset: usize) {
    indices[offset] = 0;
    indices[offset + 1] = 0;
    indices[offset + 2] = 0;
    indices[offset + 3] = 0;
    indices[offset + 4] = 0;
    indices[offset + 5] = 0;
}
