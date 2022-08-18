use std::cmp::Ordering;

use super::{caret_position::CaretPosition, rectangle::Rectangle, update_indices, update_vertices};
use crate::{
    constants::{CARET_WIDTH, SELECTION_COLOR, TEXT_PADDING},
    coordinator::dimensions::Dimensions,
    graphics::vertex::Vertex,
};

#[derive(Default)]
pub struct Selection {
    start: CaretPosition,
    end: CaretPosition,
    should_show: bool,
}

impl Selection {
    pub fn update(
        &mut self,
        dimensions: &Dimensions,
        line_widths: &[usize],
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) {
        if !self.should_show {
            return;
        }
        let (start, end) = Self::get_ordered_selection(self.start, self.end);
        let num_rows = (end.line - start.line) + 1;
        let mut row_selections = Vec::with_capacity(num_rows);
        if num_rows == 1 {
            row_selections.push((start.line, start.column, end.column));
        } else {
            if start.column < line_widths[start.line] {
                row_selections.push((start.line, start.column, line_widths[start.line]));
            }
            for i in 1..num_rows - 1 {
                let line_to_add = start.line + i;
                row_selections.push((line_to_add, 0, line_widths[line_to_add]));
            }
            if end.column > 0 {
                row_selections.push((end.line, 0, end.column));
            }
        }
        row_selections
            .into_iter()
            .enumerate()
            .for_each(|(index, (line, start_col, end_col))| {
                let top = TEXT_PADDING - dimensions.scroll()
                    + (line as f32 * (TEXT_PADDING + dimensions.glyph_height()))
                    - (TEXT_PADDING / 2.0);
                let bottom = (line + 1) as f32 * (TEXT_PADDING + dimensions.glyph_height())
                    - dimensions.scroll()
                    + (TEXT_PADDING / 2.0);
                let left =
                    CARET_WIDTH + TEXT_PADDING + (start_col as f32 * dimensions.glyph_width());
                let right = TEXT_PADDING + (end_col as f32 * dimensions.glyph_width());
                let rect = Rectangle {
                    top: dimensions.as_y_vertex(top),
                    bottom: dimensions.as_y_vertex(bottom),
                    left: dimensions.as_x_vertex(left),
                    right: dimensions.as_x_vertex(right),
                    color: SELECTION_COLOR,
                };
                update_vertices(rect, vertices, 4 + index);
            });
        for i in 0..num_rows {
            update_indices(indices, 4 + i);
        }
    }

    pub fn show_selection(
        &mut self,
        dimensions: &Dimensions,
        line_widths: &[usize],
        selection_start: CaretPosition,
        selection_end: CaretPosition,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u16>,
    ) {
        self.should_show = true;
        self.start = selection_start;
        self.end = selection_end;
        self.update(dimensions, line_widths, vertices, indices);
    }

    pub fn hide_selection(&mut self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u16>) {
        self.should_show = false;
        let vertices_to_truncate_to = 4 * 4;
        vertices.truncate(vertices_to_truncate_to);
        let indices_to_truncate_to = 4 * 6;
        indices.truncate(indices_to_truncate_to);
    }

    fn get_ordered_selection(
        left: CaretPosition,
        right: CaretPosition,
    ) -> (CaretPosition, CaretPosition) {
        match left.line.cmp(&right.line) {
            Ordering::Equal if left.column <= right.column => (left, right),
            Ordering::Equal => (right, left),
            Ordering::Less => (left, right),
            Ordering::Greater => (right, left),
        }
    }
}
