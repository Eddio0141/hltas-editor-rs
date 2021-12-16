use hltas::types::FrameBulk;
use imgui::Ui;

use crate::guis::main::cmd_editor::show_cmd_editor;

pub fn show_command_menu(ui: &Ui, framebulk: &mut FrameBulk, id: &str) -> bool {
    ui.text("commands");
    match &mut framebulk.console_command {
        Some(cmds) => show_cmd_editor(ui, cmds, &format!("##command_menu_cmds{}", id)),
        None => {
            let button_clicked = ui.button(format!("set commands##{}", id));
            if button_clicked {
                framebulk.console_command = Some(String::from(""));
            }
            button_clicked
        }
    }
}
