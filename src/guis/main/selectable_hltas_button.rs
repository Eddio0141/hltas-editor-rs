use eframe::egui::{ComboBox, Id, Ui};
use hltas::types::Button;

use crate::helpers::hltas::button_to_str;

// TODO think about this function a bit more you know
pub fn selectable_hltas_button(button: &mut Button, ui: &mut Ui, id: Id) {
    let buttons = vec![
        Button::Back,
        Button::BackLeft,
        Button::BackRight,
        Button::Forward,
        Button::ForwardLeft,
        Button::ForwardRight,
        Button::Left,
        Button::Right,
    ];

    ComboBox::from_id_source(id)
        .selected_text(button_to_str(&button))
        .show_ui(ui, |ui| {
            for button_enum in buttons {
                ui.selectable_value(button, button_enum, button_to_str(&button_enum));
            }
        });
}
