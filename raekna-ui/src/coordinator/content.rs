use std::time::Instant;

use raekna_common::{EditPosition, RCalculator};

use super::{dimensions::Dimensions, selection::Selection, text_buffer::TextBuffer};
use crate::{
    constants::TEXT_PADDING,
    graphics::controls::{caret_position::CaretPosition, Controls},
};

pub struct Content {
    pub calculator: Box<dyn RCalculator>,
    pub text_buffer: TextBuffer,
    pub selection: Selection,
    pub controls: Controls,
}

impl Content {
    pub fn new(calculator: Box<dyn RCalculator>, dimensions: &mut Dimensions) -> Self {
        let text_buffer = {
            let lines = calculator.get_all_lines();
            TextBuffer::new(lines, dimensions)
        };
        dimensions.scroll_ratio = {
            let num_lines = text_buffer.line_widths().len();
            let space_needed =
                TEXT_PADDING + num_lines as f32 * (TEXT_PADDING + dimensions.glyph_height());
            let space_available = dimensions.window_height();
            space_available / space_needed
        };
        let caret_position = CaretPosition::new(text_buffer.line_widths());
        let selection = Selection::new(caret_position);
        let controls = Controls::new(dimensions, &caret_position);

        Self {
            calculator,
            text_buffer,
            selection,
            controls,
        }
    }

    pub fn update(&mut self, dimensions: &Dimensions) {
        self.controls
            .update(dimensions, self.text_buffer.line_widths());
    }

    pub fn update_time(&mut self) -> (bool, Option<Instant>) {
        self.controls.update_time()
    }

    pub fn handle_line_updates(&mut self, dimensions: &Dimensions) {
        let lines = self.calculator.get_all_lines();
        self.text_buffer.update(lines, dimensions);
    }

    pub fn update_caret_position(&mut self, dimensions: &Dimensions) {
        self.controls
            .update_caret_position(dimensions, &self.selection.caret_position());
    }

    pub fn get_edit_selection(&self) -> (EditPosition, Option<EditPosition>) {
        let root_position = self
            .selection
            .root_position()
            .and_then(|pos| self.get_caret_position(pos));
        let caret_position = self
            .get_caret_position(self.selection.caret_position())
            .unwrap_or(CaretPosition {
                line: 0,
                column: 0,
                actual_column: 0,
            });
        let mut selection = Selection::new(caret_position);
        selection.set_root(root_position);
        match selection.as_ordered() {
            Some((start, end)) => (
                EditPosition::new(start.line, start.column),
                Some(EditPosition::new(end.line, end.column)),
            ),
            None => (
                EditPosition::new(caret_position.line, caret_position.column),
                None,
            ),
        }
    }

    pub fn handle_selection(
        &mut self,
        dimensions: &Dimensions,
        selection: (CaretPosition, Option<CaretPosition>),
    ) {
        let (start, end) = selection;
        match end {
            Some(end) => {
                self.selection.set_selection(start, end);
                let line_widths = self.text_buffer.line_widths();
                self.controls
                    .show_selection(dimensions, line_widths, start, end)
            }
            None => {
                self.selection.set_position(start);
                self.controls.hide_selection();
            }
        }
    }

    fn get_caret_position(&self, caret_position: CaretPosition) -> Option<CaretPosition> {
        let line_widths = self.text_buffer.line_widths();
        let line_counts = self.text_buffer.line_counts();
        let mut line_offset = 0;
        let mut line_number = caret_position.line;
        line_counts.iter().enumerate().find_map(|(index, count)| {
            if line_number < *count {
                let widths =
                    (line_offset..line_offset + line_number).fold(0, |sum, i| sum + line_widths[i]);
                let column = widths + caret_position.column;
                Some(CaretPosition {
                    line: index,
                    column,
                    actual_column: column,
                })
            } else {
                line_offset += *count;
                line_number -= *count;
                None
            }
        })
    }
}
