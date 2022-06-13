use std::time::Instant;

use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

use crate::constants::MULTI_CLICK_DELAY;

#[derive(Copy, Clone, Debug, Default)]
pub struct MultiClickState {
    last_click_time: Option<Instant>,
    click_count: u8,
}

impl MultiClickState {
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                if let ElementState::Pressed = state {
                    match self.last_click_time {
                        Some(time) if time.elapsed().as_millis() <= MULTI_CLICK_DELAY => {
                            self.click_count = self.click_count.saturating_add(1);
                        }
                        _ => {
                            self.click_count = 1;
                        }
                    }
                    self.last_click_time = Some(Instant::now());
                }
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::LShift | VirtualKeyCode::RShift),
                        ..
                    },
                ..
            } => {}
            _ => {
                self.reset();
            }
        }
    }

    pub fn click_count(&self) -> u8 {
        self.click_count
    }

    fn reset(&mut self) {
        self.last_click_time = None;
        self.click_count = 0;
    }
}
