use hltas::types::{DuckBeforeCollision, DuckBeforeGround, DuckWhenJump, Times};
use imgui::Ui;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, HLTASInfo};

pub struct DuckEditor;

impl FramebulkEditor for DuckEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        _: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;

        let auto_actions = &mut framebulk.auto_actions;

        let (mut duck_before_collision, mut duck_before_collision_inc_ceiling) =
            if let Some(dbc) = &auto_actions.duck_before_collision {
                (true, dbc.including_ceilings)
            } else {
                (false, false)
            };

        let mut duck_before_ground = auto_actions.duck_before_ground.is_some();

        let mut duck_when_jump = auto_actions.duck_when_jump.is_some();

        ui.text("auto duck");

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
            if duck_before_collision {
                framebulk.action_keys.duck = false;
                auto_actions.duck_before_collision = Some(DuckBeforeCollision {
                    times: Times::UnlimitedWithinFrameBulk,
                    including_ceilings: duck_before_collision_inc_ceiling,
                });
            } else {
                auto_actions.duck_before_collision = None;
            }
        }

        if duck_before_collision && inc_ceiling_changed {
            if let Some(dbc) = &mut auto_actions.duck_before_collision {
                dbc.including_ceilings = duck_before_collision_inc_ceiling;
            }
        }

        if before_ground_changed {
            if duck_before_ground {
                framebulk.action_keys.duck = false;
                auto_actions.duck_before_ground = Some(DuckBeforeGround {
                    times: Times::UnlimitedWithinFrameBulk,
                });
            } else {
                auto_actions.duck_before_ground = None;
            }
        }

        if when_jump_changed {
            if duck_when_jump {
                framebulk.action_keys.duck = false;
                auto_actions.duck_when_jump = Some(DuckWhenJump {
                    times: Times::UnlimitedWithinFrameBulk,
                });
            } else {
                auto_actions.duck_when_jump = None;
            }
        }

        before_collision_changed || inc_ceiling_changed || before_ground_changed
    }

    fn show_minimal(&self, _: &Ui, _: HLTASInfo, _: FramebulkEditorMiscData, _: usize) -> bool {
        false
    }
}
