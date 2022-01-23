use hltas::types::{AutoMovement, Line, StrafeDir};
use imgui::{Drag, StyleVar, Ui};

use crate::guis::x_button::show_x_button;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct YawPitchEditor;

impl FramebulkEditor for YawPitchEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, undo_redo_handler) =
            (misc_data.tab_menu_data, misc_data.undo_redo_handler);

        let width = 200.;

        let yaw = match &mut framebulk.auto_actions.movement {
            Some(auto_movement) => match auto_movement {
                AutoMovement::SetYaw(yaw) => Some(yaw),
                AutoMovement::Strafe(strafe_settings) => match &mut strafe_settings.dir {
                    StrafeDir::Yaw(yaw) => Some(yaw),
                    StrafeDir::Line { yaw } => Some(yaw),
                    _ => None,
                },
            },
            None => None,
        };

        let yaw_edited = match yaw {
            Some(yaw) => {
                let x_button_clicked = show_x_button(ui, &format!("yaw_set_close{}", index));
                let x_button_width = ui.item_rect_size()[0];

                let item_width_token = ui.push_style_var(StyleVar::ItemSpacing([0., 0.]));
                ui.same_line();
                item_width_token.pop();

                let item_width_token = ui.push_item_width(width - x_button_width);
                let yaw_changed = Drag::new(format!("##yaw_set{}", index))
                    .speed(0.1)
                    .display_format("yaw: %f")
                    .build(ui, yaw);
                item_width_token.pop(ui);

                if ui.is_item_activated() {
                    tab_menu_data.set_framebulk_edit_backup(framebulk, index);
                }
                if ui.is_item_deactivated_after_edit() {
                    tab_menu_data.set_undo_point_with_backup(undo_redo_handler);
                }

                if x_button_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.auto_actions.movement = None;
                }

                x_button_clicked || yaw_changed
            }
            None => {
                let disabled = match framebulk.auto_actions.movement {
                    Some(auto_movement) => match auto_movement {
                        AutoMovement::SetYaw(_) => false,
                        AutoMovement::Strafe(strafe_settings) => !matches!(
                            strafe_settings.dir,
                            StrafeDir::Yaw(_) | StrafeDir::Line { .. }
                        ),
                    },
                    None => false,
                };

                let mut edited = false;
                ui.disabled(disabled, || {
                    if ui.button_with_size(
                        format!("{}##yaw_set_button{}", "set yaw", index),
                        [width, 0.0],
                    ) {
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                        framebulk.auto_actions.movement = Some(AutoMovement::SetYaw(0.0));
                        edited = true;
                    }
                });

                edited
            }
        };

        let pitch_edited = match &mut framebulk.pitch {
            Some(pitch) => {
                let pitch_x_clicked = show_x_button(ui, &format!("pitch_set_close{}", index));
                let x_button_width = ui.item_rect_size()[0];

                let item_width_token = ui.push_style_var(StyleVar::ItemSpacing([0., 0.]));
                ui.same_line();
                item_width_token.pop();

                let item_width_token = ui.push_item_width(width - x_button_width);
                // let pitch_set_changed = Slider::new(format!("##pitch_set{}", index), -89.0, 89.0)
                //     .display_format("pitch: %f")
                //     .build(ui, pitch);
                let pitch_set_changed = Drag::new(format!("##pitch_set{}", index))
                    .display_format("pitch: %f")
                    .speed(0.1)
                    .range(-89.0, 89.0)
                    .build(ui, pitch);

                if ui.is_item_activated() {
                    tab_menu_data.set_framebulk_edit_backup(framebulk, index);
                }
                if ui.is_item_deactivated_after_edit() {
                    tab_menu_data.set_undo_point_with_backup(undo_redo_handler);
                }

                item_width_token.pop(ui);

                if pitch_x_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.pitch = None;
                }

                pitch_x_clicked || pitch_set_changed
            }
            None => {
                let pitch_set_button_clicked = ui.button_with_size(
                    format!("{}##pitch_set_button{}", "set pitch", index),
                    [width, 0.0],
                );

                if pitch_set_button_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.pitch = Some(0.0);
                }

                pitch_set_button_clicked
            }
        };

        yaw_edited || pitch_edited
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let (framebulk, properties) = (hltas_info.framebulk, hltas_info.properties);
        let (tab_menu_data, options, undo_redo_handler) = (
            misc_data.tab_menu_data,
            misc_data.options,
            misc_data.undo_redo_handler,
        );

        let yaw = match framebulk.auto_actions.movement {
            Some(auto_movement) => match auto_movement {
                AutoMovement::SetYaw(yaw) => Some(yaw),
                AutoMovement::Strafe(strafe_settings) => match strafe_settings.dir {
                    StrafeDir::Yaw(yaw) => Some(yaw),
                    StrafeDir::Line { yaw } => Some(yaw),
                    _ => None,
                },
            },
            None => None,
        };

        let angles_text = {
            let angles_text = vec![
                match yaw {
                    Some(yaw) => yaw.to_string(),
                    None => "-".to_string(),
                },
                "/".to_string(),
                match framebulk.pitch {
                    Some(pitch) => pitch.to_string(),
                    None => "-".to_string(),
                },
            ];

            angles_text.join(" ")
        };

        let yaw_pitch_popup_id = &format!("yaw_pitch_popup{}", index);
        let mut yaw_pitch_edited = false;
        ui.popup(yaw_pitch_popup_id, || {
            yaw_pitch_edited = self.show(
                ui,
                FramebulkInfo::new(framebulk, properties),
                FramebulkEditorMiscData {
                    tab_menu_data,
                    options,
                    undo_redo_handler,
                },
                index,
            );
        });

        ui.text("angles");
        ui.same_line();
        if ui.button_with_size(
            format!("{}##angles_menu_open{}", angles_text, index),
            [150., 0.],
        ) {
            ui.open_popup(yaw_pitch_popup_id);
        }

        yaw_pitch_edited
    }
}
