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
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, WindowEvent},
    event_loop::ControlFlow,
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use self::{
    content::Content,
    dimensions::Dimensions,
    input_handler::{InputHandler, multi_click_state::MultiClickState},
    user_input::{KeyboardEdit, MouseInput, UserInput},
};
use crate::{
    constants::{CARET_PERIOD, TEXT_PADDING},
    graphics::renderer_trait::{RenderBackend, WgpuRenderBackend},
};

pub struct Coordinator {
    // Order of renderer and window here is significant
    // Reordering will lead to segmentation fault on exit
    renderer: Option<WgpuRenderBackend<'static>>,
    pub window: Option<Window>,
    dimensions: Dimensions,
    content: Content,
    input_handler: InputHandler,
    multi_click_state: MultiClickState,
    close_requested: bool,
}

impl ApplicationHandler for Coordinator {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop.create_window(self.window_attributes()).unwrap();
        self.on_resumed(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(window) = &self.window
            && window.id() != window_id {
                return;
            }

        match event {
            WindowEvent::RedrawRequested => {
                match self.redraw() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => self.reconfigure_surface(),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {
                if !self.handle_window_event(&event)
                    && let WindowEvent::CloseRequested = event {
                        event_loop.exit();
                    }
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self.on_main_events_cleared() {
            Some(last_update) => {
                let next_update = last_update
                    .checked_add(std::time::Duration::from_millis(CARET_PERIOD as u64))
                    .unwrap();
                event_loop.set_control_flow(ControlFlow::WaitUntil(next_update));
            }
            None => {
                event_loop.set_control_flow(ControlFlow::Wait);
            }
        };
        self.request_redraw();

        if self.close_requested {
            event_loop.exit();
        }
    }
}

impl Coordinator {
    pub fn new(calculator: Box<dyn RCalculator>) -> Self {
        let mut dimensions = Dimensions::new();
        let content = Content::new(calculator, &mut dimensions);

        let input_handler = InputHandler::default();
        let multi_click_state = MultiClickState::default();
        let mut coordinator = Self {
            window: None,
            renderer: None,
            dimensions,
            content,
            input_handler,
            multi_click_state,
            close_requested: false,
        };
        coordinator.scroll_to_caret();
        coordinator
    }

    pub fn window_attributes(&self) -> WindowAttributes {
        Window::default_attributes()
            .with_min_inner_size(self.dimensions.minimum_window_size)
            .with_inner_size(self.dimensions.window_size)
            .with_title("raekna")
    }

    pub fn on_resumed(&mut self, window: Window) {
        // SAFETY: The window is owned by Coordinator and will live as long as the renderer
        let renderer = unsafe {
            let window_ptr = &window as *const Window;
            futures_lite::future::block_on(WgpuRenderBackend::new(
                &*window_ptr,
                &self.content.controls,
            ))
        };
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    pub fn request_redraw(&mut self) {
        let Some(window) = &self.window else {
            return;
        };

        window.request_redraw();
    }

    pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        let Some(renderer) = &mut self.renderer else {
            return Ok(());
        };

        renderer.update(&self.content.controls);
        renderer.render(&self.content, &self.dimensions)?;
        Ok(())
    }

    pub fn reconfigure_surface(&mut self) {
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let size = renderer.size();
        self.handle_resize(size);
    }

    pub fn on_main_events_cleared(&mut self) -> Option<Instant> {
        let Some(window) = &self.window else {
            return None;
        };

        let (should_redraw, last_caret_visibility_update) = self.content.update_time();

        if should_redraw {
            window.request_redraw();
        }
        last_caret_visibility_update
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool {
        self.multi_click_state.handle_event(event);
        let event_was_handled = match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
                true
            }
            WindowEvent::Resized(physical_size) => {
                self.handle_resize(*physical_size);
                true
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                inner_size_writer: _,
            } => {
                // In winit 0.30, we need to handle the new inner_size_writer
                // For now, we'll get the current size from the window
                let new_size = {
                    let Some(window) = &self.window else {
                        return false;
                    };
                    window.inner_size()
                };
                self.handle_resize(new_size);
                true
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(text) = &event.text {
                        for c in text.chars() {
                            if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                                let input = UserInput::KeyboardEdit(KeyboardEdit::Input(c));
                                self.input_handler.on_user_input(
                                    input,
                                    &mut self.content,
                                    &mut self.dimensions,
                                );
                                self.scroll_to_caret();
                            }
                        }
                    }

                    let key_code = match event.physical_key {
                        PhysicalKey::Code(code) => Some(code),
                        _ => None,
                    };
                    if let Some(input) = self.input_handler.parse_keyboard_input(&key_code) {
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
            WindowEvent::ModifiersChanged(modifiers) => {
                self.input_handler.on_modifiers_changed(&modifiers.state());
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                let Some(window) = &self.window else {
                    return false;
                };
                let position = PhysicalPosition::new(position.x as f32, position.y as f32);
                let input = UserInput::MouseInput(MouseInput::CursorMoved(position));
                self.input_handler
                    .on_user_input(input, &mut self.content, &mut self.dimensions);
                window.set_cursor(self.input_handler.cursor_icon());
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
            let Some(window) = &self.window else {
                return false;
            };
            window.request_redraw();
        }
        event_was_handled
    }

    fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        let Some(window) = &self.window else {
            return;
        };
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        if self.dimensions.update(new_size) {
            renderer.resize(self.dimensions.window_size);
            self.content.update(&self.dimensions);
            self.content
                .text_buffer
                .update_scroll(self.dimensions.scroll());
            window.request_redraw();
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
                if diff < 0.0 { 0.0 } else { diff }
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
