use hltas::types::ActionKeys;
use imgui::Ui;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct ActionKeysEditor;

fn show_action_keys_menu(ui: &Ui, action_keys: &mut ActionKeys, index: usize) -> bool {
    let use_changed = ui.checkbox(format!("use##{}", index), &mut action_keys.use_);
    let attack1_changed = ui.checkbox(format!("attack 1##{}", index), &mut action_keys.attack_1);
    let attack2_changed = ui.checkbox(format!("attack 2##{}", index), &mut action_keys.attack_2);
    let reload_changed = ui.checkbox(format!("reload##{}", index), &mut action_keys.reload);

    use_changed || attack1_changed || attack2_changed || reload_changed
}

impl FramebulkEditor for ActionKeysEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        _: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;

        let action_keys = &mut framebulk.action_keys;

        ui.text("action keys");

        show_action_keys_menu(ui, action_keys, index)
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        _: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = framebulk_info.framebulk;

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
            action_keys_edited = show_action_keys_menu(ui, action_keys, index);
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
