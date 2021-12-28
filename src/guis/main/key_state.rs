use imgui::Io;
use winit::event::VirtualKeyCode;

pub struct KeyboardState {
    pressed_prev: [bool; 512],
    pressed: [bool; 512],
    just_pressed: [bool; 512],
}

impl KeyboardState {
    // pub fn new(io: &Io) -> Self {
    //     Self {
    //         pressed_prev: [false; 512],
    //         pressed: io.keys_down,
    //         just_pressed: [false; 512],
    //     }
    // }

    pub fn update(&mut self, io: &Io) {
        self.pressed = io.keys_down;

        for (i, pressed) in self.pressed.iter().enumerate() {
            self.just_pressed[i] = *pressed && !self.pressed_prev[i];
            self.pressed_prev[i] = *pressed;
        }
    }

    pub fn held(&self, key: VirtualKeyCode) -> bool {
        self.pressed[key as usize]
    }

    pub fn just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed[key as usize]
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            pressed_prev: [false; 512],
            pressed: [false; 512],
            just_pressed: [false; 512],
        }
    }
}
