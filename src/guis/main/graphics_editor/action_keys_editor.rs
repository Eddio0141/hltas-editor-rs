use hltas::types::{FrameBulk, Line};
use imgui::Ui;

use crate::guis::main::undo_redo_hltas::UndoRedoHandler;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct ActionKeysEditor;

fn show_action_keys_menu(
    ui: &Ui,
    framebulk: &mut FrameBulk,
    undo_redo_handler: &mut UndoRedoHandler,
    index: usize,
) -> bool {
    let action_keys = &mut framebulk.action_keys;

    let use_changed = ui.checkbox(format!("use##{}", index), &mut action_keys.use_);
    let attack1_changed = ui.checkbox(format!("attack 1##{}", index), &mut action_keys.attack_1);
    let attack2_changed = ui.checkbox(format!("attack 2##{}", index), &mut action_keys.attack_2);
    let reload_changed = ui.checkbox(format!("reload##{}", index), &mut action_keys.reload);

    if use_changed {
        let mut framebulk_before = framebulk.to_owned();
        framebulk_before.action_keys.use_ = !framebulk_before.action_keys.use_;
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
    }
    if attack1_changed {
        let mut framebulk_before = framebulk.to_owned();
        framebulk_before.action_keys.attack_1 = !framebulk_before.action_keys.attack_1;
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
    }
    if attack2_changed {
        let mut framebulk_before = framebulk.to_owned();
        framebulk_before.action_keys.attack_2 = !framebulk_before.action_keys.attack_2;
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
    }
    if reload_changed {
        let mut framebulk_before = framebulk.to_owned();
        framebulk_before.action_keys.reload = !framebulk_before.action_keys.reload;
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
    }

    use_changed || attack1_changed || attack2_changed || reload_changed
}

impl FramebulkEditor for ActionKeysEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let undo_redo_handler = misc_data.undo_redo_handler;

        ui.text("action keys");

        show_action_keys_menu(ui, framebulk, undo_redo_handler, index)
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = framebulk_info.framebulk;
        let undo_redo_handler = misc_data.undo_redo_handler;

        let action_keys = &mut framebulk.action_keys;

        let width = 130.;

        let menu_button_text = {
            let mut button_text = Vec::new();

            if action_keys.use_ {
                button_text.push("use");
            }
            if action_keys.attack_1 {
                button_text.push("att1");
            }
            if action_keys.attack_2 {
                button_text.push("att2");
            }
            if action_keys.reload {
                button_text.push("rld");
            }

            if button_text.is_empty() {
                "no action key set".to_string()
            } else {
                button_text.join("/")
            }
        };

        let action_keys_popup_id = &format!("action_keys_popup{}", index);
        let mut action_keys_edited = false;
        ui.popup(action_keys_popup_id, || {
            action_keys_edited = show_action_keys_menu(ui, framebulk, undo_redo_handler, index);
        });

        if ui.button_with_size(
            format!("{}##open_action_keys_menu{}", menu_button_text, index),
            [width, 0.],
        ) {
            ui.open_popup(action_keys_popup_id);
        }

        action_keys_edited
    }
}
