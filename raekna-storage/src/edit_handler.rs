use raekna_common::{EditAction, EditPosition};

use crate::storage::Storage;

pub struct EditHandler<'a> {
    storage: &'a mut Storage,
}

impl<'a> EditHandler<'a> {
    pub fn handle_actions(storage: &'a mut Storage, actions: Vec<EditAction>) {
        let mut handler = EditHandler { storage };
        actions.into_iter().for_each(|action| match action {
            EditAction::NewLine(position) => handler.handle_newline(position),
            EditAction::Insert(position, c) => handler.handle_insert(position, c),
            EditAction::InsertMultiple(position, content) => {
                handler.handle_insert_multiple(position, content)
            }
            EditAction::Delete {
                selection_start,
                selection_end,
            } => handler.handle_delete(selection_start, selection_end),
            EditAction::DeleteForward(position) => handler.handle_delete_forward(position),
        });
    }

    fn line_at<F, R>(&mut self, index: usize, op: F) -> Option<R>
    where
        F: Fn(&mut String) -> R,
    {
        if index < self.storage.lines.len() {
            let line = &mut self.storage.lines[index];
            Some(op(line))
        } else {
            None
        }
    }

    fn handle_newline(&mut self, position: EditPosition) {
        let EditPosition { line, column } = position;
        if column == self.line_at(line, |s| s.len()).unwrap_or(0) {
            self.storage.lines.new_at(line + 1);
        } else if column == 0 {
            self.storage.lines.new_at(line);
        } else {
            let new_line = self.storage.lines[line].split_off(column);
            self.storage.lines.insert(line + 1, new_line);
        }
    }

    fn handle_insert(&mut self, position: EditPosition, c: char) {
        let EditPosition { line, column } = position;
        self.line_at(line, |s| s.insert(column, c));
    }

    fn handle_insert_multiple(&mut self, position: EditPosition, content: String) {
        let EditPosition { mut line, column } = position;
        let remaining = self
            .line_at(line, |s| s[column..].to_owned())
            .unwrap_or_else(|| "".to_owned());
        self.line_at(line, |s| s.truncate(column));
        let mut lines = content.lines();
        if let Some(line_contents) = lines.next() {
            self.line_at(line, |s| s.push_str(line_contents));
        }
        lines.for_each(|line_contents| {
            line += 1;
            self.storage.lines.insert(line, line_contents.to_owned());
        });
        self.line_at(line, |s| s.push_str(&remaining));
    }

    fn handle_delete(
        &mut self,
        selection_start: EditPosition,
        selection_end: Option<EditPosition>,
    ) {
        let start = selection_start;
        match selection_end {
            None => {
                if start.line == 0 && start.column == 0 {
                } else if start.column == 0 {
                    let removed = self.storage.lines.remove(start.line);
                    self.line_at(start.line - 1, |s| s.push_str(&removed.0));
                } else {
                    let to_remove = start.column - 1;
                    self.line_at(start.line, |s| s.remove(to_remove));
                }
            }
            Some(end) => {
                if start.line == end.line {
                    let end_str = self
                        .line_at(start.line, |s| s[end.column..].to_owned())
                        .unwrap_or_else(|| "".to_owned());
                    self.line_at(start.line, |s| s.replace_range(start.column.., &end_str));
                } else {
                    let end_line = self.storage.lines.remove(end.line);
                    let end_str = &end_line.0[end.column..];
                    self.line_at(start.line, |s| s.replace_range(start.column.., end_str));
                    for i in (start.line + 1..end.line).rev() {
                        self.storage.lines.remove(i);
                    }
                }
            }
        }
    }

    fn handle_delete_forward(&mut self, position: EditPosition) {
        let EditPosition { line, column } = position;
        let is_last_line = line == self.storage.lines.len() - 1;
        let is_last_column = column == self.line_at(line, |s| s.len()).unwrap_or(0);
        if is_last_line && is_last_column {
        } else if column == self.line_at(line, |s| s.len()).unwrap_or(0) {
            let removed = self.storage.lines.remove(line + 1);
            self.line_at(line, |s| s.push_str(&removed.0));
        } else {
            self.line_at(line, |s| s.remove(column));
        }
    }
}
