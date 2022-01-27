use hltas::types::{
    JumpBug, LeaveGroundAction, LeaveGroundActionSpeed, LeaveGroundActionType, Line, Times,
};
use imgui::{Selectable, StyleColor, Ui};

use crate::guis::main::undo_redo_hltas::UndoRedoHandler;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

struct BeforeState {
    ducktap: bool,
    zero_ms: bool,
    autojump: bool,
    jump: bool,
    duck: bool,
    jumpbug: bool,
}

struct ChangedState {
    ducktap: bool,
    zero_ms: bool,
    autojump: bool,
    jump: bool,
    duck: bool,
    jumpbug: bool,
}

fn validate_jump_states(
    hltas_info: FramebulkInfo,
    misc_data: FramebulkEditorMiscData,
    before_state: BeforeState,
    changed_state: ChangedState,
    index: usize,
) {
    let (framebulk, properties) = (hltas_info.framebulk, hltas_info.properties);
    let (options, undo_redo_handler) = (misc_data.options, misc_data.undo_redo_handler);

    // this toggles the ducktap state
    if changed_state.ducktap {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if before_state.ducktap {
            framebulk.auto_actions.leave_ground_action = None;
        } else {
            framebulk.action_keys.jump = false;
            framebulk.auto_actions.leave_ground_action = Some(LeaveGroundAction {
                speed: match framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => leave_ground_action.speed,
                    None => options.ducktap_lgagst_option().default_selection(),
                },
                times: Times::UnlimitedWithinFrameBulk,
                type_: LeaveGroundActionType::DuckTap {
                    zero_ms: properties.frametime_0ms.is_some()
                        && options.zero_ms_if_property_enabled(),
                },
            })
        }
    }

    if changed_state.zero_ms {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        match &mut framebulk.auto_actions.leave_ground_action {
            Some(leave_ground_action) => match &mut leave_ground_action.type_ {
                LeaveGroundActionType::DuckTap { zero_ms } => {
                    *zero_ms = !before_state.zero_ms;
                }
                LeaveGroundActionType::Jump => {
                    leave_ground_action.type_ = LeaveGroundActionType::DuckTap { zero_ms: true }
                }
            },
            None => {
                framebulk.action_keys.jump = false;
                framebulk.auto_actions.leave_ground_action = Some(LeaveGroundAction {
                    speed: match framebulk.auto_actions.leave_ground_action {
                        Some(leave_ground_action) => leave_ground_action.speed,
                        None => options.ducktap_lgagst_option().default_selection(),
                    },
                    times: Times::UnlimitedWithinFrameBulk,
                    type_: LeaveGroundActionType::DuckTap { zero_ms: true },
                })
            }
        }
    }

    // this toggles the jump state
    if changed_state.autojump {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if before_state.autojump {
            framebulk.auto_actions.leave_ground_action = None;
        } else {
            framebulk.action_keys.jump = false;
            framebulk.auto_actions.leave_ground_action = Some(LeaveGroundAction {
                speed: match framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => leave_ground_action.speed,
                    None => options.jump_lgagst_option().default_selection(),
                },
                times: Times::UnlimitedWithinFrameBulk,
                type_: LeaveGroundActionType::Jump,
            })
        }
    }

    // for that single "jump" selectable
    if changed_state.jump {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if !before_state.jump {
            // disable all other jump / ducktap stuff
            framebulk.auto_actions.leave_ground_action = None;
            framebulk.auto_actions.jump_bug = None;
        }
        framebulk.action_keys.jump = !before_state.jump;
    }

    // for that single "duck" selectable
    if changed_state.duck {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if !before_state.duck {
            // disable all other unused stuff
            framebulk.auto_actions.jump_bug = None;
            framebulk.auto_actions.duck_before_collision = None;
            framebulk.auto_actions.duck_before_ground = None;
            framebulk.auto_actions.duck_when_jump = None;
        }
        framebulk.action_keys.duck = !before_state.duck;
    }

    // toggle jumpbug stuff
    if changed_state.jumpbug {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if before_state.jumpbug {
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
    }
}

fn validate_lgagst_state(
    hltas_info: FramebulkInfo,
    undo_redo_handler: &mut UndoRedoHandler,
    lgagst_states: LgagstStates,
    index: usize,
) {
    let framebulk = hltas_info.framebulk;

    if lgagst_states.lgagst_changed || lgagst_states.lgagst_max_spd_changed {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
    }

    if let Some(leave_ground_action) = &mut framebulk.auto_actions.leave_ground_action {
        let lgagst_state = &mut leave_ground_action.speed;

        framebulk.auto_actions.jump_bug = None;

        // toggle lgagst
        if lgagst_states.lgagst_changed {
            if lgagst_states.lgagst_before {
                *lgagst_state = LeaveGroundActionSpeed::Any;
            } else {
                *lgagst_state = LeaveGroundActionSpeed::Optimal;
            }
        }

        // toggle lgagst max spd
        if lgagst_states.lgagst_max_spd_changed {
            if lgagst_states.lgagst_max_spd_before {
                *lgagst_state = LeaveGroundActionSpeed::Any;
            } else {
                *lgagst_state = LeaveGroundActionSpeed::OptimalWithFullMaxspeed;
            }
        }
    }
}

fn disabled_text_selectable<S>(ui: &Ui, selectable: S, grey_condition: bool) -> bool
where
    S: FnOnce(&Ui) -> bool,
{
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
}

struct LgagstStates {
    lgagst_changed: bool,
    lgagst_before: bool,
    lgagst_max_spd_changed: bool,
    lgagst_max_spd_before: bool,
}

pub struct JumpEditor;

impl FramebulkEditor for JumpEditor {
    fn show(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let (framebulk, properties) = (framebulk_info.framebulk, framebulk_info.properties);
        let (undo_redo_handler, tab_menu_data, options) = (
            misc_data.undo_redo_handler,
            misc_data.tab_menu_data,
            misc_data.options,
        );

        let starting_x = ui.cursor_pos()[0];

        let jump_ducktap_selectable_width = 65.0;

        let (autojump_before, ducktap_before, zero_ms_before) =
            match &framebulk.auto_actions.leave_ground_action {
                Some(leave_ground_action) => match leave_ground_action.type_ {
                    LeaveGroundActionType::Jump => (true, false, false),
                    LeaveGroundActionType::DuckTap { zero_ms } => (false, true, zero_ms),
                },
                None => (false, false, false),
            };

        let jump_before = framebulk.action_keys.jump;
        let duck_before = framebulk.action_keys.duck;

        let jumpbug_before = framebulk.auto_actions.jump_bug.is_some();

        ui.text("jump / ducktaps");

        ui.set_cursor_pos([starting_x, ui.cursor_pos()[1]]);
        let ducktap_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("ducktap##jump_menu{}", index))
                    .selected(ducktap_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !ducktap_before,
        );
        ui.same_line();
        let zero_ms_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("0ms##jump_menu{}", index))
                    .selected(zero_ms_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !ducktap_before,
        );

        let autojump_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("autojump##jump_menu{}", index))
                    .selected(autojump_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !autojump_before,
        );

        let jump_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("jump##jump_menu{}", index))
                    .selected(jump_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !jump_before,
        );

        ui.same_line();

        let duck_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("duck##jump_menu{}", index))
                    .selected(duck_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !duck_before,
        );

        let jumpbug_changed = disabled_text_selectable(
            ui,
            |ui| {
                Selectable::new(format!("jumpbug##jump_menu{}", index))
                    .selected(jumpbug_before)
                    .size([jump_ducktap_selectable_width, 0.0])
                    .build(ui)
            },
            !jumpbug_before,
        );

        ui.dummy([0.0, 15.0]);

        // lgagst, jumpbug selectables and state checks
        let mut lgagst_edited = false;
        ui.disabled(!ducktap_before && !autojump_before, || {
            let width = jump_ducktap_selectable_width * 2.0 + 8.0;

            let (lgagst_before, lgagst_max_spd_before) =
                match &framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => match &leave_ground_action.speed {
                        LeaveGroundActionSpeed::Any => (false, false),
                        LeaveGroundActionSpeed::Optimal => (true, false),
                        LeaveGroundActionSpeed::OptimalWithFullMaxspeed => (false, true),
                    },
                    None => (false, false),
                };

            let lgagst_changed = Selectable::new(format!("lgagst##jump_menu{}", index))
                .selected(lgagst_before)
                .size([width, 0.0])
                .build(ui);
            let lgagst_max_spd_changed =
                Selectable::new(format!("lgagst with max spd##jump_menu{}", index))
                    .selected(lgagst_max_spd_before)
                    .size([width, 0.0])
                    .build(ui);

            validate_lgagst_state(
                FramebulkInfo::new(framebulk, properties),
                undo_redo_handler,
                LgagstStates {
                    lgagst_changed,
                    lgagst_before,
                    lgagst_max_spd_changed,
                    lgagst_max_spd_before,
                },
                index,
            );

            lgagst_edited = lgagst_changed || lgagst_max_spd_changed || jumpbug_changed;
        });

        validate_jump_states(
            FramebulkInfo::new(framebulk, properties),
            FramebulkEditorMiscData {
                tab_menu_data,
                options,
                undo_redo_handler,
            },
            BeforeState {
                ducktap: ducktap_before,
                zero_ms: zero_ms_before,
                autojump: autojump_before,
                jump: jump_before,
                duck: duck_before,
                jumpbug: jumpbug_before,
            },
            ChangedState {
                ducktap: ducktap_changed,
                zero_ms: zero_ms_changed,
                autojump: autojump_changed,
                jump: jump_changed,
                duck: duck_changed,
                jumpbug: jumpbug_changed,
            },
            index,
        );

        ducktap_changed || autojump_changed || jump_changed || duck_changed || lgagst_edited
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let (framebulk, properties) = (framebulk_info.framebulk, framebulk_info.properties);
        let (undo_redo_handler, tab_menu_data, options) = (
            misc_data.undo_redo_handler,
            misc_data.tab_menu_data,
            misc_data.options,
        );

        let selectable_width = 130.;

        let jump_menu_display = {
            let mut text = Vec::new();

            match framebulk.auto_actions.leave_ground_action {
                Some(leave_ground_action) => match leave_ground_action.type_ {
                    LeaveGroundActionType::Jump => text.push("Auto Jump"),
                    LeaveGroundActionType::DuckTap { zero_ms } => {
                        if zero_ms {
                            text.push("0ms Ducktap");
                        } else {
                            text.push("Ducktap");
                        }
                    }
                },
                None => {
                    if framebulk.action_keys.jump {
                        text.push("Jump");
                    } else if framebulk.auto_actions.jump_bug.is_some() {
                        text.push("Jump Bug");
                    }
                }
            }

            if framebulk.action_keys.duck {
                text.push("Duck");
            }

            if text.is_empty() {
                "None".to_string()
            } else {
                text.join("+")
            }
        };

        let lgagst_menu_display = match framebulk.auto_actions.leave_ground_action {
            Some(leave_ground_action) => match leave_ground_action.speed {
                LeaveGroundActionSpeed::Any => "No Lgagst",
                LeaveGroundActionSpeed::Optimal => "Lgagst",
                LeaveGroundActionSpeed::OptimalWithFullMaxspeed => "Lgagst With Max Spd",
            },
            None => "No Lgagst",
        };

        let (autojump_before, ducktap_before, zero_ms_before) =
            match &framebulk.auto_actions.leave_ground_action {
                Some(leave_ground_action) => match leave_ground_action.type_ {
                    LeaveGroundActionType::Jump => (true, false, false),
                    LeaveGroundActionType::DuckTap { zero_ms } => (false, true, zero_ms),
                },
                None => (false, false, false),
            };

        let jump_before = framebulk.action_keys.jump;
        let duck_before = framebulk.action_keys.duck;

        let jumpbug_before = framebulk.auto_actions.jump_bug.is_some();

        let jump_menu_id = &format!("jump_menu_popup{}", index);
        let (
            mut ducktap_changed,
            mut zero_ms_changed,
            mut autojump_changed,
            mut jump_changed,
            mut duck_changed,
            mut jumpbug_changed,
        ) = (false, false, false, false, false, false);
        ui.popup(jump_menu_id, || {
            ducktap_changed = disabled_text_selectable(
                ui,
                |ui| {
                    Selectable::new(format!("ducktap##jump_menu{}", index))
                        .selected(ducktap_before)
                        .size([selectable_width, 0.0])
                        .build(ui)
                },
                !ducktap_before,
            );
            zero_ms_changed = disabled_text_selectable(
                ui,
                |ui| {
                    Selectable::new(format!("0ms##jump_menu{}", index))
                        .selected(zero_ms_before)
                        .size([selectable_width, 0.0])
                        .build(ui)
                },
                !ducktap_before,
            );
            autojump_changed = disabled_text_selectable(
                ui,
                |ui| {
                    Selectable::new(format!("autojump##jump_menu{}", index))
                        .selected(autojump_before)
                        .size([selectable_width, 0.0])
                        .build(ui)
                },
                !autojump_before,
            );
            jump_changed = disabled_text_selectable(
                ui,
                |ui| {
                    Selectable::new(format!("jump##jump_menu{}", index))
                        .selected(jump_before)
                        .size([selectable_width, 0.0])
                        .build(ui)
                },
                !jump_before,
            );
            jumpbug_changed = disabled_text_selectable(
                ui,
                |ui| {
                    Selectable::new(format!("jumpbug##jump_menu{}", index))
                        .selected(jumpbug_before)
                        .size([selectable_width, 0.0])
                        .build(ui)
                },
                !jumpbug_before,
            );
            let mut duck_dummy = duck_before;
            duck_changed = ui.checkbox(format!("duck##jump_menu{}", index), &mut duck_dummy);
        });

        let lgagst_menu_id = &format!("lgagst_menu_popup{}", index);
        let mut lgagst_edited = false;
        ui.popup(lgagst_menu_id, || {
            let (lgagst_before, lgagst_max_spd_before) =
                match &framebulk.auto_actions.leave_ground_action {
                    Some(leave_ground_action) => match &leave_ground_action.speed {
                        LeaveGroundActionSpeed::Any => (false, false),
                        LeaveGroundActionSpeed::Optimal => (true, false),
                        LeaveGroundActionSpeed::OptimalWithFullMaxspeed => (false, true),
                    },
                    None => (false, false),
                };

            ui.dummy([0., 5.]);
            let lgagst_changed = Selectable::new(format!("lgagst##jump_menu{}", index))
                .selected(lgagst_before)
                .size([selectable_width, 0.0])
                .build(ui);
            let lgagst_max_spd_changed =
                Selectable::new(format!("lgagst with max spd##jump_menu{}", index))
                    .selected(lgagst_max_spd_before)
                    .size([selectable_width, 0.0])
                    .build(ui);
            ui.dummy([0., 5.]);

            validate_lgagst_state(
                FramebulkInfo::new(framebulk, properties),
                undo_redo_handler,
                LgagstStates {
                    lgagst_changed,
                    lgagst_before,
                    lgagst_max_spd_changed,
                    lgagst_max_spd_before,
                },
                index,
            );

            lgagst_edited = lgagst_changed || lgagst_max_spd_changed || jumpbug_changed;
        });

        if ui.button_with_size(
            format!("{}##jump_menu_open{}", jump_menu_display, index),
            [120., 0.],
        ) {
            ui.open_popup(jump_menu_id);
        }
        ui.same_line();
        ui.disabled(!ducktap_before && !autojump_before, || {
            if ui.button_with_size(
                format!("{}##lgagst_menu_open{}", lgagst_menu_display, index),
                [150., 0.],
            ) {
                ui.open_popup(lgagst_menu_id);
            }
        });

        validate_jump_states(
            FramebulkInfo::new(framebulk, framebulk_info.properties),
            FramebulkEditorMiscData {
                tab_menu_data,
                options,
                undo_redo_handler,
            },
            BeforeState {
                ducktap: ducktap_before,
                zero_ms: zero_ms_before,
                autojump: autojump_before,
                jump: jump_before,
                duck: duck_before,
                jumpbug: jumpbug_before,
            },
            ChangedState {
                ducktap: ducktap_changed,
                zero_ms: zero_ms_changed,
                autojump: autojump_changed,
                jump: jump_changed,
                duck: duck_changed,
                jumpbug: jumpbug_changed,
            },
            index,
        );

        ducktap_changed || autojump_changed || jump_changed || duck_changed || lgagst_edited
    }
}
