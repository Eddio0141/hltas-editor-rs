use hltas::types::FrameBulk;
use imgui::Ui;

use super::framebulk_editor::FramebulkEditor;

pub struct ActionKeysEditor;

impl FramebulkEditor for ActionKeysEditor {
    fn show(
        &self,
        ui: &Ui,
        framebulk: &mut FrameBulk,
        _: &hltas::types::Properties,
        _: &mut crate::guis::main::tab::HLTASMenuState,
        _: &crate::guis::main::option_menu::AppOptions,
        index: usize,
    ) -> bool {
        let action_keys = &mut framebulk.action_keys;

        ui.text("action keys");

        let use_changed = ui.checkbox(format!("use##{}", index), &mut action_keys.use_);
        let attack1_changed =
            ui.checkbox(format!("attack 1##{}", index), &mut action_keys.attack_1);
        let attack2_changed =
            ui.checkbox(format!("attack 2##{}", index), &mut action_keys.attack_2);
        let reload_changed = ui.checkbox(format!("reload##{}", index), &mut action_keys.reload);

        use_changed || attack1_changed || attack2_changed || reload_changed
    }

    fn show_minimal(
        &self,
        _: &Ui,
        _: &mut FrameBulk,
        _: &hltas::types::Properties,
        _: &mut crate::guis::main::tab::HLTASMenuState,
        _: &crate::guis::main::option_menu::AppOptions,
        _: usize,
    ) -> bool {
        false
    }
}
