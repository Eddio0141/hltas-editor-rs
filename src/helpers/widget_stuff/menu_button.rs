use eframe::egui::{CtxRef, Key, Modifiers, Ui};
use fluent_templates::{LanguageIdentifier, Loader};

use crate::{guis::main::MainGUI, helpers::egui::key::key_to_string};

// TODO key conflict check
pub struct MenuButton<T>
where
    T: FnMut(&mut MainGUI) -> (),
{
    shortcut: Option<(Key, Modifiers)>,
    pub name: String,
    // TODO better way to call in on_click?
    on_click: T,
}

impl<T> MenuButton<T>
where
    T: FnMut(&mut MainGUI) -> (),
{
    pub fn new(
        shortcut: Option<(Key, Modifiers)>,
        translation_text_id: &str,
        language: LanguageIdentifier,
        on_click: T,
    ) -> Self {
        let mut name = crate::LOCALES
            .lookup(&language, translation_text_id)
            .to_string();

        if let Some(key_press) = shortcut {
            let key = &key_press.0;
            let modifiers = &key_press.1;

            let mut name_str_separated: Vec<String> = Vec::new();
            if modifiers.ctrl {
                name_str_separated.push("Ctrl".to_string());
            }
            if modifiers.alt {
                name_str_separated.push("Alt".to_string());
            }
            if modifiers.shift {
                name_str_separated.push("Shift".to_string());
            }
            // if modifiers.command
            if modifiers.mac_cmd {
                name_str_separated.push("⌘".to_string());
            }
            name_str_separated.push(key_to_string(key).to_string());

            name += &("      ".to_string() + &name_str_separated.join("+"));
        }

        Self {
            shortcut,
            name,
            on_click,
        }
    }

    pub fn key_check(&mut self, ctx: &CtxRef, main_gui: &mut MainGUI) {
        if let Some(key_modifiers) = &self.shortcut {
            let key = key_modifiers.0;
            let modifiers = key_modifiers.1;
            let input = ctx.input();

            if input.modifiers == modifiers && input.key_pressed(key) {
                (self.on_click)(main_gui);
            }
        }
    }

    pub fn create_button(&mut self, ui: &mut Ui, main_gui: &mut MainGUI) {
        if ui.button(&self.name).clicked() {
            (self.on_click)(main_gui);
        }
    }
}
