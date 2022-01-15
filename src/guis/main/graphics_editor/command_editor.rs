use imgui::Ui;

use crate::guis::main::cmd_editor::show_cmd_editor;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct CommandEditor;

impl FramebulkEditor for CommandEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let options = framebulk_editor_misc_data.options;

        let locale_lang = options.locale_lang();

        ui.text(locale_lang.get_string_from_id("commands"));
        match &mut framebulk.console_command {
            Some(cmds) => show_cmd_editor(
                ui,
                cmds,
                &format!("##command_menu_cmds{}", index),
                locale_lang,
            ),
            None => {
                let button_clicked = ui.button(format!("set commands##{}", index));
                if button_clicked {
                    framebulk.console_command = Some(String::from(""));
                }
                button_clicked
            }
        }
    }

    fn show_minimal(&self, _: &Ui, _: FramebulkInfo, _: FramebulkEditorMiscData, _: usize) -> bool {
        false
    }
}
