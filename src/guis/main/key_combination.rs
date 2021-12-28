use std::fmt::Display;

use winit::event::VirtualKeyCode;

use super::key_state::KeyboardState;

pub struct KeyCombination {
    key_ctrl: bool,
    key_alt: bool,
    key_shift: bool,
    key: VirtualKeyCode,
}

impl KeyCombination {
    pub fn new(key: VirtualKeyCode) -> Self {
        Self {
            key_ctrl: false,
            key_alt: false,
            key_shift: false,
            key,
        }
    }

    pub fn ctrl(self) -> Self {
        Self {
            key_ctrl: true,
            ..self
        }
    }

    // pub fn alt(self) -> Self {
    //     Self {
    //         key_alt: true,
    //         ..self
    //     }
    // }

    // pub fn shift(self) -> Self {
    //     Self {
    //         key_shift: true,
    //         ..self
    //     }
    // }

    pub fn just_pressed(&self, keyboard_state: &KeyboardState) -> bool {
        let alt_pressed =
            keyboard_state.held(VirtualKeyCode::LAlt) || keyboard_state.held(VirtualKeyCode::RAlt);
        let ctrl_pressed = keyboard_state.held(VirtualKeyCode::LControl)
            || keyboard_state.held(VirtualKeyCode::RControl);
        let shift_pressed = keyboard_state.held(VirtualKeyCode::LShift)
            || keyboard_state.held(VirtualKeyCode::RShift);

        (self.key_alt == alt_pressed)
            && (self.key_ctrl == ctrl_pressed)
            && (self.key_shift == shift_pressed)
            && keyboard_state.just_pressed(self.key)
    }
}

impl Display for KeyCombination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_string = Vec::new();

        if self.key_ctrl {
            key_string.push("ctrl");
        }
        if self.key_alt {
            key_string.push("alt");
        }
        if self.key_shift {
            key_string.push("shift");
        }

        let key = format!("{:?}", &self.key);

        key_string.push(&key);

        write!(f, "{}", key_string.join("+"))
    }
}
