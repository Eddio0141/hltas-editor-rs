use hltas::types::FrameBulk;
use imgui::Ui;

pub fn show_action_keys_menu(ui: &Ui, framebulk: &mut FrameBulk, id: &str) -> bool {
    let action_keys = &mut framebulk.action_keys;

    ui.text("action keys");

    let use_changed = ui.checkbox(format!("use##{}", id), &mut action_keys.use_);
    let attack1_changed = ui.checkbox(format!("attack 1##{}", id), &mut action_keys.attack_1);
    let attack2_changed = ui.checkbox(format!("attack 2##{}", id), &mut action_keys.attack_2);
    let reload_changed = ui.checkbox(format!("reload##{}", id), &mut action_keys.reload);

    use_changed || attack1_changed || attack2_changed || reload_changed
}
