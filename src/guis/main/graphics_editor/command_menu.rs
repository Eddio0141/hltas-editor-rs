use hltas::types::FrameBulk;
use imgui::Ui;

use crate::{guis::main::cmd_editor::show_cmd_editor, helpers::locale::locale_lang::LocaleLang};

pub fn show_command_menu(
    ui: &Ui,
    framebulk: &mut FrameBulk,
    id: &str,
    locale_lang: &LocaleLang,
) -> bool {
    ui.text(locale_lang.get_string_from_id("commands"));
    match &mut framebulk.console_command {
        Some(cmds) => show_cmd_editor(ui, cmds, &format!("##command_menu_cmds{}", id), locale_lang),
        None => {
            let button_clicked = ui.button(format!("set commands##{}", id));
            if button_clicked {
                framebulk.console_command = Some(String::from(""));
            }
            button_clicked
        }
    }
}
