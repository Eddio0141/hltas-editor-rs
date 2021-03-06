use imgui::{Condition, Ui, Window};
use winit::event::VirtualKeyCode;

use crate::helpers::{imgui::input_editor::InputUsize, locale::locale_lang::LocaleLang};

use super::{key_state::KeyboardState, tab::HLTASFileTab};

pub struct GotoMenu {
    prev_opened: bool,
    opened: bool,
    selected_index: usize,
}

impl GotoMenu {
    pub fn open(&mut self) {
        self.opened = true;
    }

    pub fn show(
        &mut self,
        ui: &Ui,
        locale_lang: &LocaleLang,
        current_tab: &mut HLTASFileTab,
        keyboard_state: &KeyboardState,
    ) {
        if self.opened {
            let mut opened_internal = true;
            let selected_index = &mut self.selected_index;
            let prev_opened = self.prev_opened;

            // don't open if no lines exist
            if current_tab.hltas_lines().is_empty() {
                self.opened = false;
                return;
            }

            // reset menu state
            if !prev_opened {
                *selected_index = 0;
            }

            Window::new(locale_lang.get_string_from_id("goto-line"))
                .opened(&mut self.opened)
                .resizable(false)
                .position_pivot([0.5, 0.5])
                .size([250.0, 100.0], Condition::Always)
                .position(
                    {
                        let display_size = ui.io().display_size;
                        [display_size[0] * 0.5, display_size[1] * 0.5]
                    },
                    Condition::Appearing,
                )
                .build(ui, || {
                    if !prev_opened {
                        ui.set_keyboard_focus_here();
                    }

                    let lines = current_tab.hltas_lines();

                    ui.text(format!("{} lines total", lines.len()));

                    InputUsize::new().auto_select_all(true).build(
                        ui,
                        "##goto_line_input",
                        selected_index,
                    );
                    // limit upper to 1 ~ lines len
                    if *selected_index < 1 {
                        *selected_index = 1;
                    } else if *selected_index > lines.len() {
                        *selected_index = lines.len();
                    }

                    if ui.button(locale_lang.get_string_from_id("jump-to-line"))
                        || keyboard_state.just_pressed(VirtualKeyCode::Return)
                    {
                        current_tab.tab_menu_data.set_goto_line(*selected_index);
                        opened_internal = false;
                    } else if keyboard_state.just_pressed(VirtualKeyCode::Escape) {
                        opened_internal = false;
                    }
                });

            if !opened_internal {
                self.opened = false;
            }
        }

        self.prev_opened = self.opened;
    }

    pub fn is_opened(&self) -> bool {
        self.opened
    }
}

impl Default for GotoMenu {
    fn default() -> Self {
        Self {
            prev_opened: Default::default(),
            opened: Default::default(),
            selected_index: 1,
        }
    }
}
