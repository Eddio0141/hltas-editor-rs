use imgui::Ui;

pub fn show_radio_button_enum<T: Copy + PartialEq>(
    ui: &Ui,
    value: &mut T,
    enums: Vec<T>,
    labels: Vec<&str>,
    id: String,
    same_line: bool,
) -> bool {
    assert_eq!(enums.len(), labels.len());

    let mut radio_button_clicked = false;
    let loop_end = enums.len();
    for i in 0..loop_end {
        if ui.radio_button(format!("{}##{}", labels[i], id), value, enums[i]) {
            radio_button_clicked = true;
        }

        if same_line && i < loop_end - 1 {
            ui.same_line();
        }
    }

    radio_button_clicked
}
