use crate::{
    coordinator::{
        active_modifiers::ActiveModifiers, content::Content, dimensions::Dimensions,
        user_input::KeyboardMovement,
    },
    graphics::controls::{caret_position::CaretPosition, get_ordered_selection},
};

#[derive(Debug, Default)]
pub struct KeyboardMovementHandler;

impl KeyboardMovementHandler {
    pub fn on_keyboard_movement(
        &self,
        movement: KeyboardMovement,
        content: &mut Content,
        dimensions: &Dimensions,
        active_modifiers: ActiveModifiers,
    ) {
        let previous_position = content.caret_position;
        let line_widths = content.text_buffer.line_widths();
        let mut selection_handled = false;
        match movement {
            KeyboardMovement::Up => match content.root_position {
                Some(root) if !active_modifiers.shift => {
                    let (start, _) = get_ordered_selection(root, content.caret_position);
                    content
                        .caret_position
                        .set_position(start.line, start.column);
                }
                _ => {
                    content.caret_position.move_up(line_widths);
                }
            },
            KeyboardMovement::Down => match content.root_position {
                Some(root) if !active_modifiers.shift => {
                    let (_, end) = get_ordered_selection(root, content.caret_position);
                    content.caret_position.set_position(end.line, end.column);
                }
                _ => {
                    content.caret_position.move_down(line_widths);
                }
            },
            KeyboardMovement::Left => match content.root_position {
                Some(root) if !active_modifiers.shift => {
                    let (start, _) = get_ordered_selection(root, content.caret_position);
                    if active_modifiers.ctrl {
                        match content
                            .calculator
                            .get_word_boundaries(content.caret_position.into())
                        {
                            Some((boundary, _)) => content
                                .caret_position
                                .set_position(boundary.line, boundary.column),
                            None => content
                                .caret_position
                                .set_position(start.line, start.column),
                        }
                    } else {
                        content
                            .caret_position
                            .set_position(start.line, start.column);
                    }
                }
                _ => {
                    if active_modifiers.ctrl {
                        match content
                            .calculator
                            .get_word_boundaries(content.caret_position.into())
                        {
                            Some((boundary, _)) => content
                                .caret_position
                                .set_position(boundary.line, boundary.column),
                            None => {
                                content.caret_position.move_left(line_widths);
                            }
                        }
                    } else {
                        content.caret_position.move_left(line_widths);
                    }
                }
            },
            KeyboardMovement::Right => match content.root_position {
                Some(root) if !active_modifiers.shift => {
                    let (_, end) = get_ordered_selection(root, content.caret_position);
                    if active_modifiers.ctrl {
                        match content
                            .calculator
                            .get_word_boundaries(content.caret_position.into())
                        {
                            Some((_, boundary)) => content
                                .caret_position
                                .set_position(boundary.line, boundary.column),
                            None => content.caret_position.set_position(end.line, end.column),
                        }
                    } else {
                        content.caret_position.set_position(end.line, end.column);
                    }
                }
                _ => {
                    if active_modifiers.ctrl {
                        match content
                            .calculator
                            .get_word_boundaries(content.caret_position.into())
                        {
                            Some((_, boundary)) => content
                                .caret_position
                                .set_position(boundary.line, boundary.column),
                            None => {
                                content.caret_position.move_right(line_widths);
                            }
                        }
                    } else {
                        content.caret_position.move_right(line_widths);
                    }
                }
            },
            KeyboardMovement::Home => {
                if active_modifiers.ctrl {
                    content.caret_position.page_up();
                } else {
                    content.caret_position.home();
                }
            }
            KeyboardMovement::End => {
                if active_modifiers.ctrl {
                    content.caret_position.page_down(line_widths);
                } else {
                    content.caret_position.end(line_widths);
                }
            }
            KeyboardMovement::PageUp => {
                content.caret_position.page_up();
            }
            KeyboardMovement::PageDown => {
                content.caret_position.page_down(line_widths);
            }
            KeyboardMovement::SelectAll => {
                let selection_start = CaretPosition {
                    line: 0,
                    column: 0,
                    actual_column: 0,
                };
                let selection_end = CaretPosition {
                    line: line_widths.len() - 1,
                    column: line_widths[line_widths.len() - 1],
                    actual_column: line_widths[line_widths.len() - 1],
                };
                let selection = (selection_start, Some(selection_end));
                content.handle_selection(dimensions, selection);
                selection_handled = true;
            }
        }
        if !selection_handled {
            Self::maybe_handle_selection(dimensions, content, active_modifiers, previous_position);
        }
        content.update_caret_position(dimensions);
    }

    fn maybe_handle_selection(
        dimensions: &Dimensions,
        content: &mut Content,
        active_modifiers: ActiveModifiers,
        previous_position: CaretPosition,
    ) -> bool {
        let mut show_selection = |start: CaretPosition, end: CaretPosition| {
            content.controls.show_selection(
                dimensions,
                content.text_buffer.line_widths(),
                start,
                end,
            )
        };

        match (content.root_position, active_modifiers.shift) {
            (None, true) => {
                content.root_position = Some(previous_position);
                show_selection(previous_position, content.caret_position);
                true
            }
            (Some(position), true) => {
                show_selection(position, content.caret_position);
                true
            }
            (Some(_), false) => {
                content.root_position = None;
                content.controls.hide_selection();
                true
            }
            _ => false,
        }
    }
}
