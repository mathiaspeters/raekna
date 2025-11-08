use winit::keyboard::ModifiersState;

#[derive(Copy, Clone, Debug, Default)]
pub struct ActiveModifiers {
    pub shift: bool,
    pub ctrl: bool,
}

impl ActiveModifiers {
    pub fn update(&mut self, state: &ModifiersState) {
        self.shift = state.shift_key();
        self.ctrl = state.control_key();
    }
}
