use imgui::{InputText, Ui};

use crate::guis::x_button::show_x_button;

use super::property_some_none_field::{property_some_none_field_ui, PropertyFieldResult};

pub fn property_string_field_ui(
    ui: &Ui,
    field: &mut Option<String>,
    chars_noblank: bool,
    label: &str,
    enable_field_button_name: &str,
    input_text_window_scale: f32,
) -> bool {
    property_some_none_field_ui(
        ui,
        field,
        String::new(),
        enable_field_button_name,
        |field_some| {
            let x_button_clicked = !show_x_button(ui, label);

            ui.same_line();

            let item_width_token =
                ui.push_item_width(ui.window_content_region_width() * input_text_window_scale);

            let input_text_edited = InputText::new(ui, label, field_some)
                .chars_noblank(chars_noblank)
                .hint(enable_field_button_name)
                .build();

            item_width_token.pop(ui);

            PropertyFieldResult {
                field_enabled: x_button_clicked,
                edited: input_text_edited,
            }
        },
    )
}
