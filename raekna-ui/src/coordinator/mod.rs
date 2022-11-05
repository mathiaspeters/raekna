mod active_modifiers;
pub mod content;
pub mod dimensions;
mod input_handler;
pub mod selection;
pub mod text_buffer;
pub mod user_input;

use std::time::Instant;

use raekna_common::RCalculator;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use self::{
    content::Content,
    dimensions::Dimensions,
    input_handler::{multi_click_state::MultiClickState, InputHandler},
    user_input::{KeyboardEdit, MouseInput, UserInput},
};
use crate::{constants::TEXT_PADDING, rendering::render_manager::RenderManager};

pub struct Coordinator {
    pub window: Window,
    renderer: RenderManager,
    dimensions: Dimensions,
    content: Content,
    input_handler: InputHandler,
    multi_click_state: MultiClickState,
}

impl Coordinator {
    pub fn new(event_loop: &EventLoop<()>, calculator: Box<dyn RCalculator>) -> Self {
        let mut dimensions = Dimensions::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(dimensions.minimum_window_size)
            .with_inner_size(dimensions.window_size)
            .with_title("raekna")
            .build(event_loop)
            .unwrap();
        let content = Content::new(calculator, &mut dimensions);
        let renderer = futures_lite::future::block_on(RenderManager::new(&window));
        let input_handler = InputHandler::default();
        let multi_click_state = MultiClickState::default();
        let mut coordinator = Self {
            window,
            renderer,
            dimensions,
            content,
            input_handler,
            multi_click_state,
        };
        coordinator.scroll_to_caret();
        coordinator
    }

    pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer
            .render(&self.content.controls, &self.content, &self.dimensions)?;
        Ok(())
    }

    pub fn reconfigure_surface(&mut self) {
        let size = self.renderer.size();
        self.handle_resize(size);
    }

    pub fn on_main_events_cleared(&mut self) -> Option<Instant> {
        let (should_redraw, last_caret_visibility_update) = self.content.update_time();
        if should_redraw {
            self.window.request_redraw();
        }
        last_caret_visibility_update
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        self.multi_click_state.handle_event(event);
        let event_was_handled = match event {
            WindowEvent::Resized(physical_size) => {
                self.handle_resize(*physical_size);
                true
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.handle_resize(**new_inner_size);
                true
            }
            WindowEvent::ReceivedCharacter(c) => {
                if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || *c == ' ' {
                    let input = UserInput::KeyboardEdit(KeyboardEdit::Input(*c));
                    self.input_handler.on_user_input(
                        input,
                        &mut self.content,
                        &mut self.dimensions,
                    );
                    self.scroll_to_caret();
                }
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    },
                ..
            } => {
                if let ElementState::Pressed = state {
                    if let Some(input) = self.input_handler.parse_keyboard_input(virtual_keycode) {
                        self.input_handler.on_user_input(
                            input,
                            &mut self.content,
                            &mut self.dimensions,
                        );
                        self.scroll_to_caret();
                    }
                }
                true
            }
            WindowEvent::ModifiersChanged(state) => {
                self.input_handler.on_modifiers_changed(state);
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = PhysicalPosition::new(position.x as f32, position.y as f32);
                let input = UserInput::MouseInput(MouseInput::CursorMoved(position));
                self.input_handler
                    .on_user_input(input, &mut self.content, &mut self.dimensions);
                self.window
                    .set_cursor_icon(self.input_handler.cursor_icon());
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let input = UserInput::from(delta);
                self.input_handler
                    .on_user_input(input, &mut self.content, &mut self.dimensions);
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let input = MouseInput::MouseClick {
                    state: *state,
                    button: *button,
                    click_count: self.multi_click_state.click_count(),
                }
                .into();
                self.input_handler
                    .on_user_input(input, &mut self.content, &mut self.dimensions);
                true
            }
            _ => false,
        };
        if event_was_handled {
            self.window.request_redraw();
        }
        event_was_handled
    }

    fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        if self.dimensions.update(new_size) {
            self.renderer.resize(self.dimensions.window_size);
            self.content.update(&self.dimensions);
            self.content
                .text_buffer
                .update_scroll(self.dimensions.scroll());
            self.window.request_redraw();
        }
    }

    fn scroll_to_caret(&mut self) {
        let (mut caret_top, mut caret_bottom) =
            self.content.controls.caret.vertical_px(&self.dimensions);
        caret_top -= TEXT_PADDING;
        caret_bottom += TEXT_PADDING;
        let mut scrolled = false;
        if caret_top < 0.0 {
            scrolled = true;
            self.dimensions
                .set_scroll(self.dimensions.scroll() + caret_top);
        } else if caret_bottom > self.dimensions.window_height() {
            scrolled = true;
            self.dimensions.set_scroll(
                self.dimensions.scroll() + caret_bottom - self.dimensions.window_height(),
            )
        }
        if scrolled {
            let max_scroll = {
                let window_height = self.dimensions.window_height();
                let content_height = window_height / self.dimensions.scroll_ratio;
                let diff = content_height - window_height;
                if diff < 0.0 {
                    0.0
                } else {
                    diff
                }
            };
            if self.dimensions.scroll() < 0.0 {
                self.dimensions.set_scroll(0.0);
            } else if self.dimensions.scroll() > max_scroll {
                self.dimensions.set_scroll(max_scroll);
            }
            self.content
                .text_buffer
                .update_scroll(self.dimensions.scroll());
            self.content.update(&self.dimensions);
        }
    }
}
