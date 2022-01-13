use hltas::types::{FrameBulk, Properties};
use imgui::Ui;

use crate::guis::{
    self,
    main::{
        cmd_editor::show_cmd_editor, option_menu::AppOptions, tab::HLTASMenuState,
        undo_redo_hltas::UndoRedoHandler,
    },
};

use super::framebulk_editor::FramebulkEditor;

pub struct CommandEditor;

impl FramebulkEditor for CommandEditor {
    fn show(
        &self,
        ui: &Ui,
        framebulk: &mut FrameBulk,
        _: &Properties,
        _: &mut HLTASMenuState,
        options: &AppOptions,
        _: &mut UndoRedoHandler,
        index: usize,
    ) -> bool {
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

    fn show_minimal(
        &self,
        _: &Ui,
        _: &mut FrameBulk,
        _: &Properties,
        _: &mut guis::main::tab::HLTASMenuState,
        _: &AppOptions,
        _: &mut UndoRedoHandler,
        _: usize,
    ) -> bool {
        false
    }
}
