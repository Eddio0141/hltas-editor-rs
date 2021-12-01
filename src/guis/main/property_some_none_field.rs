use imgui::Ui;

pub fn property_some_none_field_ui<T, E>(
    ui: &Ui,
    field: &mut Option<T>,
    field_default: T,
    enable_field_button_name: &str,
    mut some_edit: E,
) where
    E: FnMut(&mut T) -> bool,
{
    let field_enabled = match field {
        Some(demo) => some_edit(demo),
        None => ui.button(enable_field_button_name),
    };

    if field_enabled {
        if field.is_none() {
            // TODO option to auto fill with file name
            *field = Some(field_default);
        }
    } else if field.is_some() {
        *field = None;
    }
}
