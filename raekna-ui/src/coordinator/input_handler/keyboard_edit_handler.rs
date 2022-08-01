use copypasta::{ClipboardContext, ClipboardProvider};
use raekna_common::{EditAction, EditPosition};

use crate::{
    constants::TEXT_PADDING,
    coordinator::{
        active_modifiers::ActiveModifiers, content::Content, dimensions::Dimensions,
        user_input::KeyboardEdit,
    },
};

#[derive(Debug, Default)]
pub struct KeyboardEditHandler;

impl KeyboardEditHandler {
    pub fn on_keyboard_edit(
        &self,
        content: &mut Content,
        dimensions: &mut Dimensions,
        active_modifiers: ActiveModifiers,
        edit: KeyboardEdit,
    ) {
        let (selection_start, selection_end) = content.get_edit_selection();
        match edit {
            KeyboardEdit::Input(c) => {
                let mut actions = Vec::with_capacity(2);
                if selection_end.is_some() {
                    actions.push(EditAction::Delete {
                        selection_start,
                        selection_end,
                    });
                }
                actions.push(EditAction::Insert(selection_start, c));
                let line_counts = content.text_buffer.line_counts();
                let before = if line_counts.is_empty() {
                    0
                } else {
                    line_counts[selection_start.line]
                };
                Self::perform_action(content, actions, dimensions);
                let line_counts = content.text_buffer.line_counts();
                let after = if line_counts.is_empty() {
                    0
                } else {
                    line_counts[selection_start.line]
                };
                let line_widths = content.text_buffer.line_widths();
                if selection_end.is_none() {
                    content.caret_position.move_right(line_widths);
                    if after > before {
                        content.caret_position.move_right(line_widths);
                    }
                } else {
                    Self::maybe_hide_selection(
                        content,
                        selection_start.line,
                        selection_start.column + 1,
                    );
                }
            }
            KeyboardEdit::NewLine => {
                let mut actions = Vec::with_capacity(2);
                if selection_end.is_some() {
                    actions.push(EditAction::Delete {
                        selection_start,
                        selection_end,
                    });
                }
                actions.push(EditAction::NewLine(selection_start));
                Self::perform_action(content, actions, dimensions);
                if selection_end.is_none() {
                    content
                        .caret_position
                        .move_down(content.text_buffer.line_widths());
                    content.caret_position.home();
                } else {
                    Self::maybe_hide_selection(content, selection_start.line + 1, 0);
                }
            }
            KeyboardEdit::Delete => match selection_end {
                Some(selection_end) => {
                    Self::delete_selection(content, dimensions, selection_start, selection_end);
                }
                None => {
                    if active_modifiers.ctrl {
                        match content.calculator.get_word_boundaries(
                            selection_start,
                            raekna_common::BoundaryPriority::Right,
                        ) {
                            Some((_, boundary)) => Self::delete_selection(
                                content,
                                dimensions,
                                selection_start,
                                boundary,
                            ),
                            None => {
                                let actions = vec![EditAction::DeleteForward(selection_start)];
                                Self::maybe_hide_selection(
                                    content,
                                    selection_start.line,
                                    selection_start.column,
                                );
                                Self::perform_action(content, actions, dimensions);
                            }
                        }
                    } else {
                        let actions = vec![EditAction::DeleteForward(selection_start)];
                        Self::maybe_hide_selection(
                            content,
                            selection_start.line,
                            selection_start.column,
                        );
                        Self::perform_action(content, actions, dimensions);
                    }
                }
            },
            KeyboardEdit::Backspace => {
                let widths_before = content.text_buffer.line_widths();
                let mut potential_column_after = {
                    let line = content.caret_position.line;
                    if line > 0 {
                        widths_before[line - 1]
                    } else {
                        widths_before[line]
                    }
                };
                let actions = match selection_end {
                    Some(selection_end) => {
                        vec![EditAction::Delete {
                            selection_start,
                            selection_end: Some(selection_end),
                        }]
                    }
                    None => {
                        if active_modifiers.ctrl {
                            match content.calculator.get_word_boundaries(
                                selection_start,
                                raekna_common::BoundaryPriority::Left,
                            ) {
                                Some((boundary, _)) => {
                                    potential_column_after = boundary.column;
                                    vec![EditAction::Delete {
                                        selection_start: boundary,
                                        selection_end: Some(selection_start),
                                    }]
                                }
                                None => vec![EditAction::Delete {
                                    selection_start,
                                    selection_end,
                                }],
                            }
                        } else {
                            vec![EditAction::Delete {
                                selection_start,
                                selection_end,
                            }]
                        }
                    }
                };

                let before = widths_before.len();
                Self::maybe_hide_selection(content, selection_start.line, selection_start.column);
                Self::perform_action(content, actions, dimensions);
                let after = content.text_buffer.line_widths().len();
                if selection_end.is_none() && before == after {
                    content
                        .caret_position
                        .move_left(content.text_buffer.line_widths());
                } else if selection_end.is_none() {
                    let line = content.caret_position.line;
                    content
                        .caret_position
                        .set_position(line - 1, potential_column_after);
                }
            }
            KeyboardEdit::Cut => {
                let (start, end) =
                    Self::get_cut_copy_selection(content, selection_start, selection_end);
                let selection = Self::get_selection(content, start, end);
                Self::add_to_clipboard(selection);
                Self::delete_selection(content, dimensions, start, end)
            }
            KeyboardEdit::Copy => {
                let (start, end) =
                    Self::get_cut_copy_selection(content, selection_start, selection_end);
                let selection = Self::get_selection(content, start, end);
                Self::add_to_clipboard(selection);
                Self::maybe_hide_selection(content, selection_start.line, selection_start.column);
            }
            KeyboardEdit::Paste => {
                let mut actions = Vec::with_capacity(2);
                if selection_end.is_some() {
                    actions.push(EditAction::Delete {
                        selection_start,
                        selection_end,
                    });
                }
                let to_paste = Self::get_from_clipboard();
                if !to_paste.is_empty() {
                    actions.push(EditAction::InsertMultiple(
                        selection_start,
                        to_paste.clone(),
                    ));
                }
                Self::perform_action(content, actions, dimensions);
                {
                    let pasted_lines = to_paste.lines().collect::<Vec<_>>();
                    let mut line = selection_start.line;
                    let mut column = selection_start.column;
                    if pasted_lines.len() > 1 {
                        line += pasted_lines.len() - 1;
                        column = pasted_lines[pasted_lines.len() - 1].len();
                    } else {
                        column += pasted_lines[0].len();
                    }
                    content.caret_position.set_position(line, column);
                }
                Self::maybe_hide_selection(content, selection_start.line, selection_start.column);
            }
        }
        content.update_caret_position(dimensions);
    }

    fn perform_action(
        content: &mut Content,
        actions: Vec<EditAction>,
        dimensions: &mut Dimensions,
    ) {
        let line_count_before = content.text_buffer.line_widths().len();
        content.calculator.update_line(actions);
        content.handle_line_updates(dimensions);
        let line_count_after = content.text_buffer.line_widths().len();

        if line_count_before != line_count_after {
            let space_needed = {
                let line_height = TEXT_PADDING + dimensions.glyph_height();
                TEXT_PADDING + line_count_after as f32 * line_height
            };
            let space_available = dimensions.window_height();
            let scroll_ratio = space_available / space_needed;
            dimensions.scroll_ratio = scroll_ratio;
            content.update(dimensions);
        }
    }

    fn delete_selection(
        content: &mut Content,
        dimensions: &mut Dimensions,
        selection_start: EditPosition,
        selection_end: EditPosition,
    ) {
        let selection_end = Some(selection_end);
        let actions = vec![EditAction::Delete {
            selection_start,
            selection_end,
        }];
        Self::maybe_hide_selection(content, selection_start.line, selection_start.column);
        Self::perform_action(content, actions, dimensions);
    }

    fn get_cut_copy_selection(
        content: &mut Content,
        selection_start: EditPosition,
        selection_end: Option<EditPosition>,
    ) -> (EditPosition, EditPosition) {
        match selection_end {
            Some(selection_end) => (selection_start, selection_end),
            None => {
                let start = EditPosition::new(selection_start.line, 0);
                let is_last_line =
                    content.text_buffer.line_counts().iter().sum::<usize>() - 1 == start.line;
                let end = if is_last_line {
                    let last_column = content.text_buffer.line_widths()[start.line];
                    EditPosition::new(start.line, last_column)
                } else {
                    EditPosition::new(start.line + 1, 0)
                };
                (start, end)
            }
        }
    }

    fn get_selection(
        content: &mut Content,
        selection_start: EditPosition,
        selection_end: EditPosition,
    ) -> String {
        content
            .calculator
            .get_selection(selection_start, selection_end)
    }

    fn maybe_hide_selection(content: &mut Content, line: usize, column: usize) {
        if content.root_position.is_some() {
            content.controls.hide_selection();
            content.root_position = None;
            content.caret_position.set_position(line, column);
        }
    }

    fn add_to_clipboard(content: String) {
        let _ = ClipboardContext::new().map(|mut ctx| ctx.set_contents(content));
    }

    fn get_from_clipboard() -> String {
        ClipboardContext::new()
            .and_then(|mut ctx| ctx.get_contents())
            .unwrap_or_else(|_| "".to_owned())
    }
}
