mod keyboard_edit_handler;
mod keyboard_movement_handler;
mod mouse_input_handler;
pub mod multi_click_state;

use winit::{
    event::{ModifiersState, VirtualKeyCode},
    window::CursorIcon,
};

use self::{
    keyboard_edit_handler::KeyboardEditHandler, keyboard_movement_handler::KeyboardMovementHandler,
    mouse_input_handler::MouseInputHandler,
};
use super::{
    active_modifiers::ActiveModifiers, content::Content, dimensions::Dimensions,
    user_input::MouseInput,
};
use crate::coordinator::UserInput;

#[derive(Debug, Default)]
pub struct InputHandler {
    active_modifiers: ActiveModifiers,
    keyboard_movement_handler: KeyboardMovementHandler,
    keyboard_edit_handler: KeyboardEditHandler,
    mouse_input_handler: MouseInputHandler,
}

impl InputHandler {
    pub fn on_user_input(
        &mut self,
        input: UserInput,
        content: &mut Content,
        dimensions: &mut Dimensions,
    ) {
        match input {
            UserInput::KeyboardMovement(movement) => {
                content.controls.set_caret_visible();
                self.keyboard_movement_handler.on_keyboard_movement(
                    movement,
                    content,
                    dimensions,
                    self.active_modifiers,
                )
            }
            UserInput::KeyboardEdit(edit) => {
                content.controls.set_caret_visible();
                self.keyboard_edit_handler.on_keyboard_edit(
                    content,
                    dimensions,
                    self.active_modifiers,
                    edit,
                )
            }
            UserInput::MouseInput(input) => {
                if let MouseInput::MouseClick { .. } = input {
                    content.controls.set_caret_visible();
                }
                self.mouse_input_handler.on_mouse_input(
                    input,
                    dimensions,
                    content,
                    self.active_modifiers,
                )
            }
        }
    }

    pub fn parse_keyboard_input(
        &self,
        virtual_keycode: &Option<VirtualKeyCode>,
    ) -> Option<UserInput> {
        UserInput::from_key_code(virtual_keycode, &self.active_modifiers)
    }

    pub fn on_modifiers_changed(&mut self, state: &ModifiersState) {
        self.active_modifiers.update(state);
    }

    pub fn cursor_icon(&self) -> CursorIcon {
        self.mouse_input_handler.cursor_icon
    }
}
