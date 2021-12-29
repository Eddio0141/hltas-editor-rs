use imgui::Ui;

pub struct PropertyFieldResult {
    pub field_enabled: bool,
    pub edited: bool,
}

/// Used to display ui components to modify hltas property field that has
/// Some or None.
/// some_edit must return a boolean where true will keep the some edit ui
/// enabled and false to show a button that enables a button to change none
/// into some with the field_default value.
/// 
/// * Returns true or false if edited or not.
pub fn property_some_none_field_ui<T, E>(
    ui: &Ui,
    field: &mut Option<T>,
    field_default: T,
    enable_field_button_name: &str,
    mut some_edit: E,
) -> bool
where
    E: FnMut(&mut T) -> PropertyFieldResult,
{
    let field_result = match field {
        Some(demo) => some_edit(demo),
        None => PropertyFieldResult {
            field_enabled: ui.button(enable_field_button_name),
            edited: false,
        },
    };

    let edited_some_none = if field_result.field_enabled {
        if field.is_none() {
            *field = Some(field_default);
            true
        } else {
            false
        }
    } else if field.is_some() {
        *field = None;
        true
    } else {
        false
    };

    field_result.edited || edited_some_none
}
