use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta},
    keyboard::KeyCode,
};

use super::active_modifiers::ActiveModifiers;

#[derive(Copy, Clone, Debug)]
pub enum KeyboardMovement {
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    SelectAll,
}

#[derive(Copy, Clone, Debug)]
pub enum KeyboardEdit {
    Input(char),
    NewLine,
    Delete,
    Backspace,
    Cut,
    Copy,
    Paste,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseInput {
    CursorMoved(PhysicalPosition<f32>),
    LineScroll((), f32),
    PixelScroll(PhysicalPosition<f32>),
    MouseClick {
        state: ElementState,
        button: MouseButton,
        click_count: u8,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum UserInput {
    KeyboardMovement(KeyboardMovement),
    KeyboardEdit(KeyboardEdit),
    MouseInput(MouseInput),
}

impl UserInput {
    pub fn from_key_code(
        key_code: &Option<KeyCode>,
        active_modifiers: &ActiveModifiers,
    ) -> Option<Self> {
        key_code.and_then(|key_code| match key_code {
            KeyCode::Home => Some(KeyboardMovement::Home.into()),
            KeyCode::End => Some(KeyboardMovement::End.into()),
            KeyCode::PageDown => Some(KeyboardMovement::PageDown.into()),
            KeyCode::PageUp => Some(KeyboardMovement::PageUp.into()),
            KeyCode::ArrowLeft => Some(KeyboardMovement::Left.into()),
            KeyCode::ArrowUp => Some(KeyboardMovement::Up.into()),
            KeyCode::ArrowRight => Some(KeyboardMovement::Right.into()),
            KeyCode::ArrowDown => Some(KeyboardMovement::Down.into()),
            KeyCode::Enter | KeyCode::NumpadEnter => Some(KeyboardEdit::NewLine.into()),
            KeyCode::Delete => Some(KeyboardEdit::Delete.into()),
            KeyCode::Backspace => Some(KeyboardEdit::Backspace.into()),
            KeyCode::KeyX if active_modifiers.ctrl => Some(KeyboardEdit::Cut.into()),
            KeyCode::KeyC if active_modifiers.ctrl => Some(KeyboardEdit::Copy.into()),
            KeyCode::KeyV if active_modifiers.ctrl => Some(KeyboardEdit::Paste.into()),
            KeyCode::KeyA if active_modifiers.ctrl => Some(KeyboardMovement::SelectAll.into()),
            _ => None,
        })
    }
}

impl From<KeyboardMovement> for UserInput {
    fn from(k: KeyboardMovement) -> Self {
        Self::KeyboardMovement(k)
    }
}

impl From<KeyboardEdit> for UserInput {
    fn from(k: KeyboardEdit) -> Self {
        Self::KeyboardEdit(k)
    }
}

impl From<MouseInput> for UserInput {
    fn from(m: MouseInput) -> Self {
        Self::MouseInput(m)
    }
}

impl From<&MouseScrollDelta> for UserInput {
    fn from(delta: &MouseScrollDelta) -> Self {
        let mouse_input = match delta {
            MouseScrollDelta::LineDelta(_, y) => MouseInput::LineScroll((), *y),
            MouseScrollDelta::PixelDelta(delta) => {
                let delta = PhysicalPosition::new(delta.x as f32, delta.y as f32);
                MouseInput::PixelScroll(delta)
            }
        };
        mouse_input.into()
    }
}
