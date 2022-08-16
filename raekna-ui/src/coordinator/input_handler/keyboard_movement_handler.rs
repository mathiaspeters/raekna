use raekna_common::BoundaryPriority;

use crate::{
    coordinator::{
        active_modifiers::ActiveModifiers, content::Content, dimensions::Dimensions,
        user_input::KeyboardMovement,
    },
    graphics::controls::caret_position::CaretPosition,
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
        let previous_position = content.selection.caret_position();
        let line_widths = content.text_buffer.line_widths();
        let mut selection_handled = false;
        match movement {
            KeyboardMovement::Up => match content.selection.as_ordered() {
                Some((start, _)) if !active_modifiers.shift => {
                    content.selection.update_position(|cp| {
                        cp.set_position(start.line, start.column);
                    });
                }
                _ => {
                    content.selection.update_position(|cp| {
                        cp.move_up(line_widths);
                    });
                }
            },
            KeyboardMovement::Down => match content.selection.as_ordered() {
                Some((_, end)) if !active_modifiers.shift => {
                    content.selection.update_position(|cp| {
                        cp.set_position(end.line, end.column);
                    });
                }
                _ => {
                    content.selection.update_position(|cp| {
                        cp.move_down(line_widths);
                    });
                }
            },
            KeyboardMovement::Left => match content.selection.as_ordered() {
                Some((start, _)) if !active_modifiers.shift => {
                    if active_modifiers.ctrl {
                        match content.calculator.get_word_boundaries(
                            (content.selection.caret_position()).into(),
                            BoundaryPriority::Left,
                        ) {
                            Some((boundary, _)) => {
                                content.selection.update_position(|cp| {
                                    cp.set_position(boundary.line, boundary.column)
                                });
                            }
                            None => content
                                .selection
                                .update_position(|cp| cp.set_position(start.line, start.column)),
                        }
                    } else {
                        content.selection.update_position(|cp| {
                            cp.set_position(start.line, start.column);
                        });
                    }
                }
                _ => {
                    if active_modifiers.ctrl {
                        match content.calculator.get_word_boundaries(
                            (content.selection.caret_position()).into(),
                            BoundaryPriority::Left,
                        ) {
                            Some((boundary, _)) => {
                                content.selection.update_position(|cp| {
                                    cp.set_position(boundary.line, boundary.column)
                                });
                            }
                            None => {
                                content.selection.update_position(|cp| {
                                    cp.move_left(line_widths);
                                });
                            }
                        }
                    } else {
                        content.selection.update_position(|cp| {
                            cp.move_left(line_widths);
                        });
                    }
                }
            },
            KeyboardMovement::Right => match content.selection.as_ordered() {
                Some((_, end)) if !active_modifiers.shift => {
                    if active_modifiers.ctrl {
                        match content.calculator.get_word_boundaries(
                            (content.selection.caret_position()).into(),
                            BoundaryPriority::Right,
                        ) {
                            Some((_, boundary)) => {
                                content.selection.update_position(|cp| {
                                    cp.set_position(boundary.line, boundary.column)
                                });
                            }
                            None => content
                                .selection
                                .update_position(|cp| cp.set_position(end.line, end.column)),
                        }
                    } else {
                        content.selection.update_position(|cp| {
                            cp.set_position(end.line, end.column);
                        });
                    }
                }
                _ => {
                    if active_modifiers.ctrl {
                        match content.calculator.get_word_boundaries(
                            (content.selection.caret_position()).into(),
                            BoundaryPriority::Right,
                        ) {
                            Some((_, boundary)) => {
                                content.selection.update_position(|cp| {
                                    cp.set_position(boundary.line, boundary.column)
                                });
                            }
                            None => {
                                content.selection.update_position(|cp| {
                                    cp.move_right(line_widths);
                                });
                            }
                        }
                    } else {
                        content.selection.update_position(|cp| {
                            cp.move_right(line_widths);
                        });
                    }
                }
            },
            KeyboardMovement::Home => {
                content.selection.update_position(|cp| {
                    if active_modifiers.ctrl {
                        cp.page_up();
                    } else {
                        cp.home();
                    }
                });
            }
            KeyboardMovement::End => {
                content.selection.update_position(|cp| {
                    if active_modifiers.ctrl {
                        cp.page_down(line_widths);
                    } else {
                        cp.end(line_widths);
                    }
                });
            }
            KeyboardMovement::PageUp => {
                content.selection.update_position(|cp| {
                    cp.page_up();
                });
            }
            KeyboardMovement::PageDown => {
                content.selection.update_position(|cp| {
                    cp.page_down(line_widths);
                });
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

        match (content.selection.root_position(), active_modifiers.shift) {
            (None, true) => {
                content.selection.set_root(Some(previous_position));
                show_selection(previous_position, content.selection.caret_position());
                true
            }
            (Some(position), true) => {
                show_selection(position, content.selection.caret_position());
                true
            }
            (Some(_), false) => {
                content.selection.set_root(None);
                content.controls.hide_selection();
                true
            }
            _ => false,
        }
    }
}
