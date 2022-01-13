use hltas::types::{AutoMovement, Line, StrafeDir};
use imgui::{Drag, InputFloat, Slider, StyleVar, Ui};

use crate::guis::x_button::show_x_button;

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, HLTASInfo};

pub struct YawPitchEditor;

impl FramebulkEditor for YawPitchEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, undo_redo_handler) = (
            framebulk_editor_misc_data.tab_menu_data,
            framebulk_editor_misc_data.undo_redo_handler,
        );

        let initial_x_pos = ui.cursor_pos()[0];
        let width = ui.window_content_region_width() * 0.2;

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

                ui.same_line();

                ui.set_cursor_screen_pos([
                    initial_x_pos + x_button_width,
                    ui.cursor_screen_pos()[1],
                ]);

                let item_width_token = ui.push_item_width(width - x_button_width);
                let yaw_changed = Drag::new(format!("##yaw_set{}", index))
                    .speed(0.1)
                    .display_format("yaw: %f")
                    .build(ui, yaw);
                item_width_token.pop(ui);

                if ui.is_item_active() {
                    tab_menu_data.is_modifying_framebulk(framebulk, index);
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
                    ui.set_cursor_screen_pos([initial_x_pos, ui.cursor_screen_pos()[1]]);

                    if ui.button_with_size(
                        format!("{}##yaw_set_button{}", "set yaw", index),
                        [width, 0.0],
                    ) {
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

                ui.same_line();

                ui.set_cursor_screen_pos([
                    initial_x_pos + x_button_width,
                    ui.cursor_screen_pos()[1],
                ]);

                let item_width_token = ui.push_item_width(width - x_button_width);
                let pitch_set_changed = Slider::new(format!("##pitch_set{}", index), -89.0, 89.0)
                    .display_format("pitch: %f")
                    .build(ui, pitch);
                item_width_token.pop(ui);

                if pitch_x_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.pitch = None;
                }

                pitch_x_clicked || pitch_set_changed
            }
            None => {
                ui.set_cursor_screen_pos([initial_x_pos, ui.cursor_screen_pos()[1]]);

                let pitch_set_button_clicked = ui.button_with_size(
                    format!("{}##pitch_set_button{}", "set pitch", index),
                    [width, 0.0],
                );

                if pitch_set_button_clicked {
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
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let undo_redo_handler = framebulk_editor_misc_data.undo_redo_handler;

        let width = ui.window_content_region_width() * 0.15;

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
                let x_spacing = ui.clone_style().item_spacing[0];

                ui.text("yaw");
                let yaw_text_width = ui.item_rect_size()[0];

                ui.same_line();

                let no_spacing_token = ui.push_style_var(StyleVar::ItemSpacing([0.0, 0.0]));
                let x_button_clicked = show_x_button(ui, &format!("yaw_set_close{}", index));
                let x_button_width = ui.item_rect_size()[0];

                ui.same_line();

                let width_token =
                    ui.push_item_width(width - x_button_width - yaw_text_width - x_spacing);
                let yaw_edited = InputFloat::new(ui, format!("##yaw_set{}", index), yaw).build();
                width_token.pop(ui);
                no_spacing_token.pop();

                if x_button_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.auto_actions.movement = None;
                }

                yaw_edited || x_button_clicked
            }
            None => {
                let set_yaw_pressed =
                    ui.button_with_size(format!("set yaw##{}", index), [width, 0.0]);
                if set_yaw_pressed {
                    framebulk.auto_actions.movement = Some(AutoMovement::SetYaw(0.0));
                }
                set_yaw_pressed
            }
        };

        ui.same_line();

        let pitch_edited = match &mut framebulk.pitch {
            Some(pitch) => {
                let x_spacing = ui.clone_style().item_spacing[0];

                ui.text("pitch");
                let pitch_text_width = ui.item_rect_size()[0];

                ui.same_line();

                let x_before_x_button = ui.cursor_pos()[0];
                let x_button_clicked = show_x_button(ui, &format!("pitch_set_close{}", index));
                let x_button_width = ui.item_rect_size()[0];

                ui.same_line();
                ui.set_cursor_pos([x_before_x_button + x_button_width, ui.cursor_pos()[1]]);

                let width_token =
                    ui.push_item_width(width - x_button_width - pitch_text_width - x_spacing);
                let pitch_edited =
                    InputFloat::new(ui, format!("##pitch_set{}", index), pitch).build();
                width_token.pop(ui);

                if x_button_clicked {
                    undo_redo_handler.edit_line(Line::FrameBulk(framebulk.to_owned()), index);
                    framebulk.pitch = None;
                }

                pitch_edited || x_button_clicked
            }
            None => {
                let set_pitch_pressed =
                    ui.button_with_size(format!("set pitch##{}", index), [width, 0.0]);
                if set_pitch_pressed {
                    framebulk.pitch = Some(0.0);
                }
                set_pitch_pressed
            }
        };

        yaw_edited || pitch_edited
    }
}
