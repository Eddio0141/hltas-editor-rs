use imgui::{Condition, Ui, Window};

use crate::helpers::{imgui::input_usize, locale::locale_lang::LocaleLang};

use super::tab::HLTASFileTab;

pub struct GotoMenu {
    opened: bool,
    selected_index: usize,
}

impl GotoMenu {
    pub fn open(&mut self) {
        self.opened = true;
    }

    pub fn show(&mut self, ui: &Ui, locale_lang: &LocaleLang, current_tab: &mut HLTASFileTab) {
        if self.opened {
            let mut opened_internal = true;
            let mut selected_index = &mut self.selected_index;

            Window::new(locale_lang.get_string_from_id("goto-line"))
                .opened(&mut self.opened)
                .resizable(false)
                .position_pivot([0.5, 0.5])
                .size([250.0, 90.0], Condition::Always)
                .position(
                    {
                        let display_size = ui.io().display_size;
                        [display_size[0] * 0.5, display_size[1] * 0.5]
                    },
                    Condition::Appearing,
                )
                .build(ui, || {
                    input_usize(ui, "goto line", &mut selected_index);
                    if ui.button(locale_lang.get_string_from_id("jump-to-line")) {
                        current_tab.tab_menu_data.set_goto_line(*selected_index);
                        opened_internal = false;
                    }
                });

            if !opened_internal {
                self.opened = false;
            }
        }
    }
}

impl Default for GotoMenu {
    fn default() -> Self {
        Self {
            opened: false,
            selected_index: 0,
        }
    }
}
