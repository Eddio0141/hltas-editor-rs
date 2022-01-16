use std::num::NonZeroU32;

use imgui::{Drag, Ui};

use crate::helpers::hltas::{fps, frametime};

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct FramesEditor;

impl FramebulkEditor for FramesEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        _: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;

        let frametime = framebulk.frame_time.parse::<f32>();
        let mut frame_count = framebulk.frame_count.get();

        let width_token = ui.push_item_width(150.0);

        let mut frametime_changed = false;
        // TODO error display instead (like a popup?)
        ui.disabled(frametime.is_err(), || {
            let mut frametime = frametime.unwrap_or_default();
            let mut fps = 1.0 / frametime;

            frametime_changed = Drag::new(format!("##frames_menu_frametime_drag{}", index))
                .range(frametime::MAX_STRAFE, frametime::MIN)
                .speed(0.0001)
                .display_format("frametime: %.6f")
                .build(ui, &mut frametime);

            let fps_changed = Drag::new(format!("##frames_menu_fps_drag{}", index))
                .range(fps::MIN, fps::MAX_STRAFE)
                .speed(0.01)
                .display_format("fps: %.2f")
                .build(ui, &mut fps);

            if frametime_changed {
                framebulk.frame_time = frametime.to_string();
            }
            if fps_changed {
                frametime_changed = true;
                framebulk.frame_time = (1.0 / fps).to_string();
            }
        });

        let frame_count_changed = Drag::new(format!("##frames_menu_frame_count_drag{}", index))
            .range(1, u32::MAX)
            .speed(0.1)
            .display_format("frames: %u")
            .build(ui, &mut frame_count);

        if frame_count_changed {
            if let Some(frame_count) = NonZeroU32::new(frame_count) {
                framebulk.frame_count = frame_count;
            }
        }

        width_token.pop(ui);

        frametime_changed || frame_count_changed
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = framebulk_info.framebulk;
        let tab_menu_data = misc_data.tab_menu_data;

        let frametime = framebulk.frame_time.parse::<f32>();
        let mut frame_count = framebulk.frame_count.get();

        let fps_toggle_button_name = if tab_menu_data.simple_view_show_fps() {
            "fps"
        } else {
            "ft"
        };

        if ui.button(format!(
            "{}##fps_toggle_button{}",
            fps_toggle_button_name, index
        )) {
            tab_menu_data.set_simple_view_show_fps(!tab_menu_data.simple_view_show_fps());
        }

        ui.same_line();

        let width_token = ui.push_item_width(if tab_menu_data.simple_view_show_fps() {
            100.
        } else {
            150.
        });

        let mut frametime_changed = false;
        ui.disabled(frametime.is_err(), || {
            let mut frametime = frametime.unwrap_or_default();

            if tab_menu_data.simple_view_show_fps() {
                let mut fps = 1.0 / frametime;

                let frametime_changed = Drag::new(format!("##frames_menu_fps_drag{}", index))
                    .range(fps::MIN, fps::MAX_STRAFE)
                    .speed(0.01)
                    .display_format("fps: %.2f")
                    .build(ui, &mut fps);

                if frametime_changed {
                    framebulk.frame_time = (1.0 / fps).to_string();
                }
            } else {
                frametime_changed = Drag::new(format!("##frames_menu_frametime_drag{}", index))
                    .range(frametime::MAX_STRAFE, frametime::MIN)
                    .speed(0.0001)
                    .display_format("frametime: %.6f")
                    .build(ui, &mut frametime);

                if frametime_changed {
                    framebulk.frame_time = frametime.to_string();
                }
            }
        });
        width_token.pop(ui);

        ui.same_line();

        let width_token = ui.push_item_width(110.);
        let frame_count_changed = Drag::new(format!("##frames_menu_frame_count_drag{}", index))
            .range(1, u32::MAX)
            .speed(0.1)
            .display_format("frames: %u")
            .build(ui, &mut frame_count);
        width_token.pop(ui);

        if frame_count_changed {
            if let Some(frame_count) = NonZeroU32::new(frame_count) {
                framebulk.frame_count = frame_count;
            }
        }

        frametime_changed
    }
}
