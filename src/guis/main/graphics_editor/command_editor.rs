use hltas::types::Line;
use imgui::Ui;

use crate::guis::main::cmd_editor::show_cmd_editor_undo_redo_line;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct CommandEditor;

impl FramebulkEditor for CommandEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, options, undo_redo_handler) = (
            misc_data.tab_menu_data,
            misc_data.options,
            misc_data.undo_redo_handler,
        );

        let locale_lang = options.locale_lang();

        ui.text(locale_lang.get_string_from_id("commands"));
        match framebulk.console_command {
            Some(_) => show_cmd_editor_undo_redo_line(
                ui,
                framebulk,
                &format!("##command_menu_cmds{}", index),
                FramebulkEditorMiscData::new(tab_menu_data, options, undo_redo_handler),
                index,
            ),
            None => {
                let button_clicked = ui.button(format!("set commands##{}", index));
                if button_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.console_command = Some(String::from(""));
                }
                button_clicked
            }
        }
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, options, undo_redo_handler) = (
            misc_data.tab_menu_data,
            misc_data.options,
            misc_data.undo_redo_handler,
        );

        match &mut framebulk.console_command {
            Some(_) => show_cmd_editor_undo_redo_line(
                ui,
                framebulk,
                &format!("##command_menu_cmds{}", index),
                FramebulkEditorMiscData::new(tab_menu_data, options, undo_redo_handler),
                index,
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
}
