use imgui::Ui;

pub fn show_combo_enum<T: Copy + PartialEq>(
    ui: &Ui,
    value: &mut T,
    label_enum_pairs: Vec<(&str, T)>,
    id: &str,
) -> bool {
    let (labels, enums): (Vec<_>, Vec<_>) = label_enum_pairs.iter().copied().unzip();

    let mut current_item = enums
        .iter()
        .position(|e| value == e)
        .expect("Unreachable code, `value` has the same generic type `T` as enums vector does and somehow `value` isn't found in the enums vector");

    let list_box_changed = ui.combo_simple_string(format!("##{}", id), &mut current_item, &labels);

    if list_box_changed {
        *value = enums[current_item];
    }

    list_box_changed
}
