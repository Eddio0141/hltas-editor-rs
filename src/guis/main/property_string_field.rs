use imgui::{InputText, Ui};

pub fn property_string_field_ui(
    ui: &Ui,
    field: &mut Option<String>,
    input_text_hint: &str,
    enable_field_button_name: &str,
) {
    let field_enabled = match field {
        Some(demo) => {
            ui.group(|| {
                InputText::new(ui, " ", demo)
                    .chars_noblank(true)
                    .hint(input_text_hint)
                    .build();

                ui.same_line();

                // TODO find proper "x" button
                !ui.button("x")
            })
        }
        None => ui.button(enable_field_button_name),
    };

    if field_enabled {
        if field.is_none() {
            // TODO option to auto fill with file name
            *field = Some(String::new());
        }
    } else if field.is_some() {
        *field = None;
    }
}
