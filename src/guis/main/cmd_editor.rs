use hltas::types::FrameBulk;
use imgui::{InputText, Ui};

use crate::helpers::locale::locale_lang::LocaleLang;

use super::graphics_editor::framebulk_editor::FramebulkEditorMiscData;

pub fn show_cmd_editor(ui: &Ui, cmds: &mut String, label: &str, locale_lang: &LocaleLang) -> bool {
    // TODO
    InputText::new(ui, label, cmds)
        .hint(locale_lang.get_string_from_id("commands"))
        .build()
}

pub fn show_cmd_editor_undo_redo_line(
    ui: &Ui,
    framebulk: &mut FrameBulk,
    label: &str,
    misc_data: FramebulkEditorMiscData,
    index: usize,
) -> bool {
    // TODO
    if let Some(cmds) = &mut framebulk.console_command {
        let (locale_lang, tab_menu_data, undo_redo_handler) = (
            misc_data.options.locale_lang(),
            misc_data.tab_menu_data,
            misc_data.undo_redo_handler,
        );

        let edited = InputText::new(ui, label, cmds)
            .hint(locale_lang.get_string_from_id("commands"))
            .build();

        // TODO fix this
        if ui.is_item_activated() {
            println!("activated");
            tab_menu_data.set_framebulk_edit_backup(framebulk, index)
        }
        if ui.is_item_deactivated_after_edit() {
            println!("deactivated");
            tab_menu_data.set_undo_point_with_backup(undo_redo_handler)
        }

        edited
    } else {
        false
    }
}
