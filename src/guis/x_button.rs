use imgui::Ui;

pub fn show_x_button(ui: &Ui, id: &str) -> bool {
    ui.button(format!("x##{}", id))
}
