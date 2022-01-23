use hltas::types::{DuckBeforeCollision, DuckBeforeGround, DuckWhenJump, Line, Times};
use imgui::Ui;

use crate::guis::main::undo_redo_hltas::UndoRedoHandler;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct DuckEditor;

fn auto_duck_menu(
    ui: &Ui,
    framebulk_info: FramebulkInfo,
    undo_redo_handler: &mut UndoRedoHandler,
    index: usize,
) -> bool {
    let framebulk = framebulk_info.framebulk;

    let (mut duck_before_collision, mut duck_before_collision_inc_ceiling) =
        if let Some(dbc) = &framebulk.auto_actions.duck_before_collision {
            (true, dbc.including_ceilings)
        } else {
            (false, false)
        };

    let mut duck_before_ground = framebulk.auto_actions.duck_before_ground.is_some();

    let mut duck_when_jump = framebulk.auto_actions.duck_when_jump.is_some();

    let before_collision_changed = ui.checkbox(
        format!("before collision##{}", index),
        &mut duck_before_collision,
    );

    ui.indent();

    let mut inc_ceiling_changed = false;
    ui.disabled(!duck_before_collision, || {
        inc_ceiling_changed = ui.checkbox(
            format!("+ ceiling##{}", index),
            &mut duck_before_collision_inc_ceiling,
        );
    });

    ui.unindent();

    let before_ground_changed =
        ui.checkbox(format!("before ground##{}", index), &mut duck_before_ground);

    let when_jump_changed = ui.checkbox(format!("when jump##{}", index), &mut duck_when_jump);

    if before_collision_changed {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if duck_before_collision {
            framebulk.action_keys.duck = false;
            framebulk.auto_actions.duck_before_collision = Some(DuckBeforeCollision {
                times: Times::UnlimitedWithinFrameBulk,
                including_ceilings: duck_before_collision_inc_ceiling,
            });
        } else {
            framebulk.auto_actions.duck_before_collision = None;
        }
    }

    if duck_before_collision && inc_ceiling_changed {
        if framebulk.auto_actions.duck_before_collision.is_some() {
            undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        }
        if let Some(dbc) = &mut framebulk.auto_actions.duck_before_collision {
            dbc.including_ceilings = duck_before_collision_inc_ceiling;
        }
    }

    if before_ground_changed {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if duck_before_ground {
            framebulk.action_keys.duck = false;
            framebulk.auto_actions.duck_before_ground = Some(DuckBeforeGround {
                times: Times::UnlimitedWithinFrameBulk,
            });
        } else {
            framebulk.auto_actions.duck_before_ground = None;
        }
    }

    if when_jump_changed {
        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
        if duck_when_jump {
            framebulk.action_keys.duck = false;
            framebulk.auto_actions.duck_when_jump = Some(DuckWhenJump {
                times: Times::UnlimitedWithinFrameBulk,
            });
        } else {
            framebulk.auto_actions.duck_when_jump = None;
        }
    }

    before_collision_changed || inc_ceiling_changed || before_ground_changed
}

impl FramebulkEditor for DuckEditor {
    fn show(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let (framebulk, properties) = (framebulk_info.framebulk, framebulk_info.properties);
        let undo_redo_handler = misc_data.undo_redo_handler;

        ui.text("auto duck");
        auto_duck_menu(
            ui,
            FramebulkInfo::new(framebulk, properties),
            undo_redo_handler,
            index,
        )
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let (framebulk, properties) = (framebulk_info.framebulk, framebulk_info.properties);
        let undo_redo_handler = misc_data.undo_redo_handler;

        let menu_button_size = [120., 0.];

        let button_display = {
            let mut duck_state = Vec::new();

            if let Some(duck_before_collision) = framebulk.auto_actions.duck_before_collision {
                if duck_before_collision.including_ceilings {
                    duck_state.push("dbc ceil");
                } else {
                    duck_state.push("dbc");
                }
            }
            if framebulk.auto_actions.duck_before_ground.is_some() {
                duck_state.push("dbg");
            }
            if framebulk.auto_actions.duck_when_jump.is_some() {
                duck_state.push("dwj");
            };

            if duck_state.is_empty() {
                "no auto duck".to_string()
            } else {
                duck_state.join("/")
            }
        };

        let menu_id = &format!("duck_menu_popup{}", index);

        let mut menu_edited = false;
        ui.popup(menu_id, || {
            menu_edited = auto_duck_menu(
                ui,
                FramebulkInfo::new(framebulk, properties),
                undo_redo_handler,
                index,
            );
        });

        if ui.button_with_size(
            format!("{}##duck_menu_open{}", button_display, index),
            menu_button_size,
        ) {
            ui.open_popup(menu_id);
        }

        menu_edited
    }
}
