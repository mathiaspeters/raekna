use raekna_common::{BoundaryPriority, EditPosition};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
    window::CursorIcon,
};

use crate::{
    constants::{SCROLLBAR_WIDTH_MULTIPLIER, TEXT_PADDING},
    coordinator::{
        active_modifiers::ActiveModifiers, content::Content, dimensions::Dimensions,
        selection::Selection, user_input::MouseInput,
    },
    graphics::controls::{caret_position::CaretPosition, ScrollHandleState},
};

#[derive(Debug, Default)]
pub struct MouseInputHandler {
    mouse_position: PhysicalPosition<f32>,
    mouse_click_position: CaretPosition,
    mouse_is_clicked: bool,
    selection: Selection,
    pub cursor_icon: CursorIcon,
}

impl MouseInputHandler {
    pub fn on_mouse_input(
        &mut self,
        input: MouseInput,
        dimensions: &mut Dimensions,
        content: &mut Content,
        active_modifiers: ActiveModifiers,
    ) {
        match input {
            MouseInput::CursorMoved(new_position) => {
                self.update_mouse_position(content, dimensions, new_position)
            }
            MouseInput::LineScroll(_, y) => self.handle_scroll_event(content, dimensions, y),
            MouseInput::PixelScroll(delta) => {
                self.handle_scroll_event(content, dimensions, delta.y)
            }
            MouseInput::MouseClick {
                state,
                button,
                click_count,
            } => self.handle_mouse_click(
                content,
                dimensions,
                active_modifiers,
                button,
                state,
                click_count,
            ),
        }
    }

    fn update_mouse_position(
        &mut self,
        content: &mut Content,
        dimensions: &mut Dimensions,
        new_position: PhysicalPosition<f32>,
    ) {
        let y_before = self.mouse_position.y;
        self.mouse_position = new_position;
        let y_after = self.mouse_position.y;

        self.cursor_icon = {
            let editable_width = {
                let scrollbar_width = dimensions.glyph_width() * SCROLLBAR_WIDTH_MULTIPLIER;
                dimensions.window_width() - scrollbar_width
            };
            if new_position.x < editable_width {
                CursorIcon::Text
            } else {
                CursorIcon::Default
            }
        };

        match content.controls.scrollbar.handle_state {
            ScrollHandleState::Clicked => {
                let delta = y_after - y_before;
                let new_scroll_position = dimensions.scroll() + delta;
                let window_height = dimensions.window_height();
                let cursor_is_within_scrollable_area = y_before >= 0.0
                    && y_after >= 0.0
                    && y_before <= window_height
                    && y_after <= window_height;
                if cursor_is_within_scrollable_area {
                    let max_scroll = {
                        let content_height = window_height / dimensions.scroll_ratio;
                        let diff = content_height - window_height;
                        if diff < 0.0 {
                            0.0
                        } else {
                            diff
                        }
                    };
                    dimensions.set_scroll(if new_scroll_position < 0.0 {
                        0.0
                    } else if new_scroll_position > max_scroll {
                        max_scroll
                    } else {
                        new_scroll_position
                    });
                    content.text_buffer.update_scroll(dimensions.scroll());
                }
            }
            _ => {
                if self.mouse_is_clicked {
                    let mut start_position = self.mouse_click_position;
                    let mut end_position = self.get_line_and_column(dimensions);
                    let line_widths = content.text_buffer.line_widths();
                    self.normalize_caret_position(&mut start_position, line_widths);
                    self.normalize_caret_position(&mut end_position, line_widths);
                    if self.set_selection(start_position, end_position) {
                        content.handle_selection(dimensions, self.selection);
                        content.update_caret_position(dimensions);
                    }
                }
                if content
                    .controls
                    .is_in_scroll_handle(dimensions, new_position)
                {
                    content
                        .controls
                        .update_handle_state(ScrollHandleState::Hover);
                } else {
                    content
                        .controls
                        .update_handle_state(ScrollHandleState::Default);
                }
            }
        }
        content.update(dimensions);
    }

    fn handle_mouse_click(
        &mut self,
        content: &mut Content,
        dimensions: &mut Dimensions,
        active_modifiers: ActiveModifiers,
        button: MouseButton,
        state: ElementState,
        click_count: u8,
    ) {
        let is_in_scroll_bar = content
            .controls
            .is_in_scroll_bar(dimensions, self.mouse_position);
        let is_in_scroll_handle = content
            .controls
            .is_in_scroll_handle(dimensions, self.mouse_position);

        let line_widths = content.text_buffer.line_widths();
        if let MouseButton::Left = button {
            match state {
                ElementState::Pressed => {
                    self.mouse_is_clicked = true;
                    if is_in_scroll_handle {
                        content
                            .controls
                            .update_handle_state(ScrollHandleState::Clicked);
                    } else if is_in_scroll_bar {
                        content
                            .controls
                            .update_handle_state(ScrollHandleState::Clicked);
                        let window_height = dimensions.window_height();
                        let max_scroll = {
                            let content_height = window_height / dimensions.scroll_ratio;
                            let diff = content_height - window_height;
                            if diff < 0.0 {
                                0.0
                            } else {
                                diff
                            }
                        };
                        let attempted_scroll = {
                            let scrollbar_height = window_height - max_scroll;
                            self.mouse_position.y - (scrollbar_height / 2.0)
                        };
                        dimensions.set_scroll(attempted_scroll.clamp(0.0, max_scroll));
                        content.text_buffer.update_scroll(dimensions.scroll());
                    } else {
                        if click_count >= 4 {
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
                            self.selection.update_position(|cp| *cp = selection_start);
                            self.set_selection(selection_start, selection_end);
                        } else if click_count == 3 {
                            let mut clicked_position = self.get_line_and_column(dimensions);
                            self.normalize_caret_position(&mut clicked_position, line_widths);
                            let is_last_line = clicked_position.line == line_widths.len() - 1;
                            if !active_modifiers.shift {
                                self.selection.update_position(|cp| {
                                    *cp = CaretPosition {
                                        line: clicked_position.line,
                                        column: 0,
                                        actual_column: 0,
                                    }
                                });
                            }
                            let selection_end = if is_last_line {
                                CaretPosition {
                                    line: line_widths.len() - 1,
                                    column: line_widths[line_widths.len() - 1],
                                    actual_column: line_widths[line_widths.len() - 1],
                                }
                            } else {
                                CaretPosition {
                                    line: clicked_position.line + 1,
                                    column: 0,
                                    actual_column: 0,
                                }
                            };
                            self.set_selection(self.selection.caret_position(), selection_end);
                        } else if click_count == 2 {
                            let clicked_position = self.get_line_and_column(dimensions);
                            let clicked_position =
                                EditPosition::new(clicked_position.line, clicked_position.column);
                            let word_boundaries = content
                                .calculator
                                .get_word_boundaries(clicked_position, BoundaryPriority::None);
                            match word_boundaries {
                                Some((boundary_start, boundary_end)) => {
                                    if active_modifiers.shift {
                                        todo!()
                                    } else {
                                        self.selection.set_selection(
                                            boundary_start.into(),
                                            boundary_end.into(),
                                        );
                                    }
                                }
                                None => {
                                    self.mouse_click_position =
                                        self.get_line_and_column(dimensions);
                                    let mut clicked_position = self.mouse_click_position;
                                    self.normalize_caret_position(
                                        &mut clicked_position,
                                        line_widths,
                                    );
                                    self.selection.set_position(clicked_position);
                                }
                            }
                        } else if active_modifiers.shift {
                            self.mouse_click_position = content.selection.caret_position();
                            let mut clicked_position = self.get_line_and_column(dimensions);
                            self.normalize_caret_position(&mut clicked_position, line_widths);
                            self.set_selection(
                                content.selection.caret_position(),
                                clicked_position,
                            );
                        } else {
                            self.mouse_click_position = self.get_line_and_column(dimensions);
                            let mut clicked_position = self.mouse_click_position;
                            self.normalize_caret_position(&mut clicked_position, line_widths);
                            self.selection.set_position(clicked_position);
                        }
                        content.handle_selection(dimensions, self.selection);
                        content.update_caret_position(dimensions);
                    }
                }
                ElementState::Released => {
                    if is_in_scroll_handle {
                        content
                            .controls
                            .update_handle_state(ScrollHandleState::Hover);
                    } else {
                        content
                            .controls
                            .update_handle_state(ScrollHandleState::Default);
                    }
                    self.mouse_is_clicked = false;
                }
            }
        }
        if let (MouseButton::Left, ElementState::Released) = (button, state) {
            content
                .controls
                .update_handle_state(ScrollHandleState::Default);
        }
        content.update(dimensions);
    }

    fn handle_scroll_event(
        &mut self,
        content: &mut Content,
        dimensions: &mut Dimensions,
        scroll_delta: f32,
    ) {
        let line_widths = content.text_buffer.line_widths();
        let overflow = {
            let num_lines = line_widths.len();
            let line_height = TEXT_PADDING + dimensions.glyph_height();
            let space_needed = TEXT_PADDING + num_lines as f32 * line_height;
            let space_available = dimensions.window_height();
            space_needed - space_available
        };
        let scrolling_handled = overflow > 0.0;
        if scrolling_handled {
            let mut total_scroll = dimensions.scroll() + (scroll_delta * -10.0);
            if total_scroll > overflow {
                total_scroll = overflow;
            } else if total_scroll < 0.0 {
                total_scroll = 0.0;
            }
            dimensions.set_scroll(total_scroll);
            content.controls.update(dimensions, line_widths);
            content.text_buffer.update_scroll(dimensions.scroll());
        }
    }

    fn get_line_and_column(&self, dimensions: &Dimensions) -> CaretPosition {
        let line = {
            let glyph_height = dimensions.glyph_height();
            let offset_mouse_pos = self.mouse_position.y + dimensions.scroll() - glyph_height / 2.0;
            let total_line_height = TEXT_PADDING + glyph_height;
            (offset_mouse_pos / total_line_height).round() as usize
        };
        let column = {
            let glyph_width = dimensions.glyph_width();
            let offset_mouse_pos = self.mouse_position.x - TEXT_PADDING;
            (offset_mouse_pos / glyph_width).round() as usize
        };
        CaretPosition {
            line,
            column,
            actual_column: column,
        }
    }

    fn normalize_caret_position(&self, position: &mut CaretPosition, line_widths: &[usize]) {
        let line = if position.line >= line_widths.len() {
            line_widths.len().saturating_sub(1)
        } else {
            position.line
        };
        let column = if line_widths.is_empty() {
            0
        } else if position.column > line_widths[line] {
            line_widths[line]
        } else {
            position.column
        };
        position.set_position(line, column);
    }

    fn set_selection(
        &mut self,
        start_position: CaretPosition,
        end_position: CaretPosition,
    ) -> bool {
        if start_position == end_position {
            self.selection.set_root(None);
            false
        } else if matches!(self.selection.root_position(), Some(selection_end) if selection_end == end_position)
        {
            false
        } else {
            self.selection.set_root(Some(end_position));
            true
        }
    }
}
