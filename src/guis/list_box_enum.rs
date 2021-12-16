use imgui::Ui;

pub fn show_list_box_enum<T: Copy + PartialEq>(
    ui: &Ui,
    value: &mut T,
    enums: Vec<T>,
    labels: Vec<&str>,
    id: String,
) -> bool {
    assert_eq!(enums.len(), labels.len());

    let mut current_item = enums
        .iter()
        .position(|e| value == e)
        .expect("Unreachable code, `value` has the same generic type `T` as enums vector does and somehow `value` isn't found in the enums vector") as i32;

    let list_box_changed = ui.list_box(
        format!("##{}", id),
        &mut current_item,
        &labels,
        enums.len() as i32,
    );

    if list_box_changed {
        *value = enums[current_item as usize];
    }

    list_box_changed
}
