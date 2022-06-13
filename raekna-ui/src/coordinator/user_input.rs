use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, VirtualKeyCode},
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
    LineScroll(f32, f32),
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
        key_code: &Option<VirtualKeyCode>,
        active_modifiers: &ActiveModifiers,
    ) -> Option<Self> {
        key_code.and_then(|key_code| match key_code {
            VirtualKeyCode::Home => Some(KeyboardMovement::Home.into()),
            VirtualKeyCode::End => Some(KeyboardMovement::End.into()),
            VirtualKeyCode::PageDown => Some(KeyboardMovement::PageDown.into()),
            VirtualKeyCode::PageUp => Some(KeyboardMovement::PageUp.into()),
            VirtualKeyCode::Left => Some(KeyboardMovement::Left.into()),
            VirtualKeyCode::Up => Some(KeyboardMovement::Up.into()),
            VirtualKeyCode::Right => Some(KeyboardMovement::Right.into()),
            VirtualKeyCode::Down => Some(KeyboardMovement::Down.into()),
            VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                Some(KeyboardEdit::NewLine.into())
            }
            VirtualKeyCode::Delete => Some(KeyboardEdit::Delete.into()),
            VirtualKeyCode::Back => Some(KeyboardEdit::Backspace.into()),
            VirtualKeyCode::X if active_modifiers.ctrl => Some(KeyboardEdit::Cut.into()),
            VirtualKeyCode::C if active_modifiers.ctrl => Some(KeyboardEdit::Copy.into()),
            VirtualKeyCode::V if active_modifiers.ctrl => Some(KeyboardEdit::Paste.into()),
            VirtualKeyCode::A if active_modifiers.ctrl => Some(KeyboardMovement::SelectAll.into()),
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
            MouseScrollDelta::LineDelta(x, y) => MouseInput::LineScroll(*x, *y),
            MouseScrollDelta::PixelDelta(delta) => {
                let delta = PhysicalPosition::new(delta.x as f32, delta.y as f32);
                MouseInput::PixelScroll(delta)
            }
        };
        mouse_input.into()
    }
}
