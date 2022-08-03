use raekna_common::{
    errors::{CommonError, CommonResult},
    BoundaryPriority, EditAction, EditPosition,
};

use crate::{edit_handler::EditHandler, lines::Lines, word_boundaries::find_word_boundaries};

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

    pub fn get_word_boundaries(
        &self,
        origin: EditPosition,
        priority: BoundaryPriority,
    ) -> Option<(EditPosition, EditPosition)> {
        let EditPosition { line, column } = origin;
        if line >= self.lines.len() {
            None
        } else {
            find_word_boundaries(&self.lines[line], column, priority)
                .map(|(start, end)| (EditPosition::new(line, start), EditPosition::new(line, end)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_handle_actions {
        use super::*;

        #[test]
        fn single_addition() {
            let mut sut = Storage::default();

            let actions = vec![EditAction::Insert(EditPosition::new(0, 0), 'a')];
            sut.handle_actions(actions);

            assert_eq!(sut.lines.content, vec!["a".to_owned()]);
        }

        #[test]
        fn multiple_additions() {
            let mut sut = Storage::default();

            let actions = vec![
                EditAction::Insert(EditPosition::new(0, 0), 'a'),
                EditAction::InsertMultiple(EditPosition::new(0, 1), "bc".to_owned()),
                EditAction::NewLine(EditPosition::new(0, 2)),
                EditAction::InsertMultiple(EditPosition::new(1, 1), "d".to_owned()),
                EditAction::NewLine(EditPosition::new(1, 2)),
                EditAction::InsertMultiple(EditPosition::new(2, 0), "ef".to_owned()),
                EditAction::Insert(EditPosition::new(2, 2), 'g'),
            ];
            sut.handle_actions(actions);

            assert_eq!(
                sut.lines.content,
                vec!["ab".to_owned(), "cd".to_owned(), "efg".to_owned()]
            );
        }

        #[test]
        fn insert_and_delete() {
            let mut sut = Storage {
                lines: Lines {
                    content: vec!["abc".to_owned()],
                    results: vec!["".to_owned()],
                },
            };

            let actions = vec![EditAction::Delete {
                selection_start: EditPosition::new(0, 1),
                selection_end: None,
            }];
            sut.handle_actions(actions);

            assert_eq!(sut.lines.content, vec!["bc".to_owned()]);

            let actions = vec![EditAction::DeleteForward(EditPosition::new(0, 0))];
            sut.handle_actions(actions);

            assert_eq!(sut.lines.content, vec!["c".to_owned()]);
        }

        #[test]
        fn delete_single_line_selection() {
            let mut sut = Storage {
                lines: Lines {
                    content: vec!["abcd".to_owned()],
                    results: vec!["".to_owned()],
                },
            };

            let actions = vec![EditAction::Delete {
                selection_start: EditPosition::new(0, 1),
                selection_end: Some(EditPosition::new(0, 3)),
            }];
            sut.handle_actions(actions);

            assert_eq!(sut.lines.content, vec!["ad".to_owned()]);
        }

        #[test]
        fn delete_multi_line_selection() {
            let mut sut = Storage {
                lines: Lines {
                    content: vec!["abc".to_owned(), "def".to_owned()],
                    results: vec!["".to_owned(), "".to_owned()],
                },
            };

            let actions = vec![EditAction::Delete {
                selection_start: EditPosition::new(0, 2),
                selection_end: Some(EditPosition::new(1, 2)),
            }];
            sut.handle_actions(actions);

            assert_eq!(sut.lines.content, vec!["abf".to_owned()]);
        }
    }

    mod test_get_word_boundaries {
        use super::*;

        #[test]
        fn origin_line_out_of_bounds() {
            let lines = Lines {
                content: vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
                results: vec![],
            };
            let storage = Storage { lines };

            let origin = EditPosition::new(3, 2);
            let actual = storage.get_word_boundaries(origin, BoundaryPriority::None);

            assert_eq!(actual, None);
        }

        #[test]
        fn normal_usage() {
            let lines = Lines {
                content: vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()],
                results: vec![],
            };
            let storage = Storage { lines };

            let origin = EditPosition::new(1, 2);
            let actual = storage.get_word_boundaries(origin, BoundaryPriority::None);

            let expected = (EditPosition::new(1, 0), EditPosition::new(1, 3));
            let expected = Some(expected);

            assert_eq!(actual, expected);
        }
    }
}
