use raekna_common::{
    errors::{CommonError, CommonResult},
    EditAction, EditPosition,
};

use crate::{edit_handler::EditHandler, lines::Lines};

#[derive(Debug, Default)]
pub struct Storage {
    pub lines: Lines,
}

impl Storage {
    pub fn get_lines(&self) -> (&[String], &[String]) {
        (&self.lines.content, &self.lines.results)
    }

    pub fn get_lines_mut(&mut self) -> (&[String], &mut [String]) {
        (&self.lines.content, &mut self.lines.results)
    }

    pub fn get_line(&self, index: usize) -> CommonResult<(&str, &str)> {
        if index >= self.lines.len() {
            return Err(CommonError::OutOfBounds(index));
        }
        Ok((&self.lines.content[index], &self.lines.results[index]))
    }

    pub fn handle_actions(&mut self, actions: Vec<EditAction>) {
        EditHandler::handle_actions(self, actions)
    }

    /// Assumes start and end are ordered
    pub fn get_selection(
        &self,
        selection_start: EditPosition,
        selection_end: EditPosition,
    ) -> String {
        if selection_start.line == selection_end.line {
            self.lines[selection_start.line][selection_start.column..selection_end.column]
                .to_owned()
        } else if selection_start.line == selection_end.line - 1 {
            let start_line = &self.lines[selection_start.line][selection_start.column..];
            let end_line = &self.lines[selection_end.line][..selection_end.column];
            format!("{}\n{}", start_line, end_line)
        } else {
            let mut lines = Vec::with_capacity(selection_end.line - selection_start.line + 1);
            lines.push(&self.lines[selection_start.line][selection_start.column..]);
            for i in selection_start.line + 1..selection_end.line {
                lines.push(&self.lines[i]);
            }
            lines.push(&self.lines[selection_end.line][..selection_end.column]);
            lines.join("\n")
        }
    }
}
