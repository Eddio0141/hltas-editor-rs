use imgui::{InputText, Ui};

use super::property_some_none_field::property_some_none_field_ui;

pub fn property_string_field_ui(
    ui: &Ui,
    field: &mut Option<String>,
    chars_noblank: bool,
    label: &str,
    enable_field_button_name: &str,
    input_text_window_scale: f32,
) {
    property_some_none_field_ui(
        ui,
        field,
        String::new(),
        enable_field_button_name,
        |field_some| {
            let item_width_token =
                ui.push_item_width(ui.window_content_region_width() * input_text_window_scale);

            InputText::new(ui, label, field_some)
                .chars_noblank(chars_noblank)
                .hint(enable_field_button_name)
                .build();

            item_width_token.pop(ui);

            ui.same_line();

            // TODO find proper "x" button
            !ui.button(format!("x##{}", label))
        },
    );
}
