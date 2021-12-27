use hltas::types::{
    FrameBulk, JumpBug, LeaveGroundAction, LeaveGroundActionSpeed, LeaveGroundActionType, Times,
};
use imgui::{Selectable, StyleColor, Ui};

use crate::guis::main::option_menu::AppOptions;

pub fn show_jump_menu(ui: &Ui, framebulk: &mut FrameBulk, id: &str, options: &AppOptions) -> bool {
    let jump_ducktap_selectable_width = 65.0;
    let disabled_text_selectable = |selectable: &dyn Fn(&Ui) -> bool, grey_condition: bool| {
        let color_token = if !grey_condition {
            None
        } else {
            Some(ui.push_style_color(StyleColor::Text, ui.style_color(StyleColor::TextDisabled)))
        };

        let selectable_changed = selectable(ui);

        if let Some(color_token) = color_token {
            color_token.pop();
        }

        selectable_changed
    };

    let (autojump_before, ducktap_before) = match &framebulk.auto_actions.leave_ground_action {
        Some(leave_ground_action) => match leave_ground_action.type_ {
            LeaveGroundActionType::Jump => (true, false),
            LeaveGroundActionType::DuckTap { .. } => (false, true),
        },
        None => (false, false),
    };

    let jump_before = framebulk.action_keys.jump;
    let duck_before = framebulk.action_keys.duck;

    let jumpbug_before = framebulk.auto_actions.jump_bug.is_some();

    ui.text("jump / ducktaps");

    let duck_tap_changed = disabled_text_selectable(
        &|ui| {
            Selectable::new(format!("ducktap##jump_menu{}", id))
                .selected(ducktap_before)
                .size([jump_ducktap_selectable_width, 0.0])
                .build(ui)
        },
        !ducktap_before,
    );

    ui.same_line();

    let jump_changed = disabled_text_selectable(
        &|ui| {
            Selectable::new(format!("jump##jump_menu{}", id))
                .selected(jump_before)
                .size([jump_ducktap_selectable_width, 0.0])
                .build(ui)
        },
        !jump_before,
    );

    let autojump_changed = disabled_text_selectable(
        &|ui| {
            Selectable::new(format!("autojump##jump_menu{}", id))
                .selected(autojump_before)
                .size([jump_ducktap_selectable_width, 0.0])
                .build(ui)
        },
        !autojump_before,
    );

    ui.same_line();

    let duck_changed = disabled_text_selectable(
        &|ui| {
            Selectable::new(format!("duck##jump_menu{}", id))
                .selected(duck_before)
                .size([jump_ducktap_selectable_width, 0.0])
                .build(ui)
        },
        !duck_before,
    );

    let jumpbug_changed = disabled_text_selectable(
        &|ui| {
            Selectable::new(format!("jumpbug##jump_menu{}", id))
                .selected(jumpbug_before)
                .size([jump_ducktap_selectable_width, 0.0])
                .build(ui)
        },
        !jumpbug_before,
    );

    ui.dummy([0.0, 15.0]);

    // lgagst, jumpbug selectables and state checks
    let mut lgagst_changed = false;
    ui.disabled(!ducktap_before && !autojump_before, || {
        let width = jump_ducktap_selectable_width * 2.0 + 8.0;

        let lgagst_state = match &mut framebulk.auto_actions.leave_ground_action {
            Some(leave_ground_action) => Some(&mut leave_ground_action.speed),
            None => None,
        };
        let (lgagst_enabled, lgagst_max_spd_enabled) = match &lgagst_state {
            Some(leave_ground_action_speed) => match &leave_ground_action_speed {
                LeaveGroundActionSpeed::Any => (false, false),
                LeaveGroundActionSpeed::Optimal => (true, false),
                LeaveGroundActionSpeed::OptimalWithFullMaxspeed => (false, true),
            },
            None => (false, false),
        };

        let lgagst_selected = Selectable::new(format!("lgagst##jump_menu{}", id))
            .selected(lgagst_enabled)
            .size([width, 0.0])
            .build(ui);
        let lgagst_max_spd_selected =
            Selectable::new(format!("lgagst with max spd##jump_menu{}", id))
                .selected(lgagst_max_spd_enabled)
                .size([width, 0.0])
                .build(ui);

        if jumpbug_changed {
            // toggle jumpbug stuff
            if jumpbug_before {
                framebulk.auto_actions.jump_bug = None;
            } else {
                // we need both of those keys so
                framebulk.action_keys.jump = false;
                framebulk.action_keys.duck = false;

                framebulk.auto_actions.leave_ground_action = None;
                framebulk.auto_actions.jump_bug = Some(JumpBug {
                    times: Times::UnlimitedWithinFrameBulk,
                });
            }
        } else if let Some(lgagst_state) = lgagst_state {
            framebulk.auto_actions.jump_bug = None;

            // toggle lgagst
            if lgagst_selected {
                if lgagst_enabled {
                    *lgagst_state = LeaveGroundActionSpeed::Any;
                } else {
                    *lgagst_state = LeaveGroundActionSpeed::Optimal;
                }
            }

            // toggle lgagst max spd
            if lgagst_max_spd_selected {
                if lgagst_max_spd_enabled {
                    *lgagst_state = LeaveGroundActionSpeed::Any;
                } else {
                    *lgagst_state = LeaveGroundActionSpeed::OptimalWithFullMaxspeed;
                }
            }
        }

        lgagst_changed = lgagst_selected || lgagst_max_spd_selected || jumpbug_changed;
    });

    // this toggles the ducktap state
    if duck_tap_changed {
        if ducktap_before {
            framebulk.auto_actions.leave_ground_action = None;
        } else {
            framebulk.action_keys.jump = false;
            // TODO 0ms detector or option to have 0ms by default
            framebulk.auto_actions.leave_ground_action = Some(LeaveGroundAction {
                speed: match framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => leave_ground_action.speed,
                    None => {
                        // TODO copy framebulk option thing
                        options.ducktap_lgagst_option().default_selection()
                    }
                },
                times: Times::UnlimitedWithinFrameBulk,
                type_: LeaveGroundActionType::DuckTap { zero_ms: true },
            })
        }
    }

    // this toggles the jump state
    if autojump_changed {
        if autojump_before {
            framebulk.auto_actions.leave_ground_action = None;
        } else {
            framebulk.action_keys.jump = false;
            framebulk.auto_actions.leave_ground_action = Some(LeaveGroundAction {
                speed: match framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => leave_ground_action.speed,
                    None => {
                        // TODO copy framebulk option thing
                        options.jump_lgagst_option().default_selection()
                    }
                },
                times: Times::UnlimitedWithinFrameBulk,
                type_: LeaveGroundActionType::Jump,
            })
        }
    }

    // for that single "jump" selectable
    if jump_changed {
        if !jump_before {
            // disable all other jump / ducktap stuff
            framebulk.auto_actions.leave_ground_action = None;
            framebulk.auto_actions.jump_bug = None;
        }
        framebulk.action_keys.jump = !jump_before;
    }

    // for that single "duck" selectable
    if duck_changed {
        if !duck_before {
            // disable all other unused stuff
            framebulk.auto_actions.jump_bug = None;
        }
        framebulk.action_keys.duck = !duck_before;
    }

    duck_tap_changed || autojump_changed || jump_changed || duck_changed
}
