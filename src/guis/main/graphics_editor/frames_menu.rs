use std::num::NonZeroU32;

use hltas::types::FrameBulk;
use imgui::{Drag, Ui};

use crate::helpers::hltas::{fps, frametime};

pub fn show_frames_menu(ui: &Ui, framebulk: &mut FrameBulk, id: &str) -> bool {
    let frametime = framebulk.frame_time.parse::<f32>();
    let mut frame_count = framebulk.frame_count.get();

    let width_token = ui.push_item_width(150.0);

    let mut frametime_changed = false;
    // TODO error display instead (like a popup?)
    ui.disabled(frametime.is_err(), || {
        let mut frametime = frametime.unwrap_or_default();
        let mut fps = 1.0 / frametime;

        frametime_changed = Drag::new(format!("##frames_menu_frametime_drag{}", id))
            .range(frametime::MAX_STRAFE, frametime::MIN)
            .speed(0.0001)
            .display_format("frametime: %.6f")
            .build(ui, &mut frametime);

        let fps_changed = Drag::new(format!("##frames_menu_fps_drag{}", id))
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

    let frame_count_changed = Drag::new(format!("##frames_menu_frame_count_drag{}", id))
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