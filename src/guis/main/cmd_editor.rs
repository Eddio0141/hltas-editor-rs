use imgui::{InputText, Ui};

pub fn cmd_editor_ui(ui: &Ui, cmds: &mut String, label: &str) -> bool {
    // TODO
    InputText::new(ui, label, cmds).hint("commands").build()
}
