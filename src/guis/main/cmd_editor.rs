use imgui::{InputText, Ui};

pub fn show_cmd_editor(ui: &Ui, cmds: &mut String, label: &str) -> bool {
    // TODO
    InputText::new(ui, label, cmds).hint("commands").build()
}
