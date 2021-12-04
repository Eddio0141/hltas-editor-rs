use imgui::Ui;

pub fn show_radio_button_enum<T: Copy + PartialEq>(
    ui: &Ui,
    value: &mut T,
    enums: Vec<T>,
    labels: Vec<&str>,
    same_line: bool,
) {
    assert_eq!(enums.len(), labels.len());

    let loop_end = enums.len();
    for i in 0..loop_end {
        ui.radio_button(labels[i], value, enums[i]);

        if same_line {
            ui.same_line();
        }
    }
}
