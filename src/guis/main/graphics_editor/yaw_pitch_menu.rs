use hltas::types::{AutoMovement, StrafeDir, FrameBulk};
use imgui::{Ui, Drag, Slider};

use crate::guis::x_button::show_x_button;

pub fn show_yaw_pitch_menu(ui: &Ui, yaw_pitch_menu_offset: f32, framebulk: &mut FrameBulk, id: &str) -> bool {
    let yaw_pitch_setter_width = ui.window_content_region_width() * 0.2;

    let yaw_editor = |yaw| {
        let x_button_clicked = show_x_button(ui, &format!("yaw_set_close{}", id));

        ui.same_line();

        ui.set_cursor_screen_pos([yaw_pitch_menu_offset, ui.cursor_screen_pos()[1]]);

        let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
        let yaw_set_changed = Drag::new(format!("##yaw_set{}", id))
            .speed(0.1)
            .display_format("yaw: %f")
            .build(ui, yaw);
        item_width_token.pop(ui);

        if x_button_clicked {
            None
        } else {
            Some(yaw_set_changed)
        }
    };
    let yaw_button = |disabled, auto_movement: &mut Option<AutoMovement>| {
        // ui.disabled returns nothing so hacky work around
        let mut edited = false;
        ui.disabled(disabled, || {
            ui.set_cursor_screen_pos([yaw_pitch_menu_offset, ui.cursor_screen_pos()[1]]);

            if ui.button_with_size(
                format!("{}##yaw_set_button{}", "set yaw", id),
                [yaw_pitch_setter_width, 0.0],
            ) {
                *auto_movement = Some(AutoMovement::SetYaw(0.0));
                edited = true;
            }
        });
        Some(edited)
    };

    let edited_yaw = match &mut framebulk.auto_actions.movement {
        Some(auto_movement) => match auto_movement {
            AutoMovement::SetYaw(yaw) => yaw_editor(yaw),
            AutoMovement::Strafe(strafe_settings) => match &mut strafe_settings.dir {
                StrafeDir::Yaw(yaw) => yaw_editor(yaw),
                StrafeDir::Line { yaw } => yaw_editor(yaw),
                _ => yaw_button(true, &mut framebulk.auto_actions.movement),
            },
        },
        None => {
            // show yaw button
            yaw_button(false, &mut framebulk.auto_actions.movement)
        }
    };

    let edited_yaw = match edited_yaw {
        Some(edited_yaw) => edited_yaw,
        None => {
            framebulk.auto_actions.movement = None;
            true
        }
    };

    let edited_pitch = match &mut framebulk.pitch {
        Some(pitch) => {
            let pitch_x_clicked = show_x_button(ui, &format!("pitch_set_close{}", id));

            ui.same_line();

            ui.set_cursor_screen_pos([yaw_pitch_menu_offset, ui.cursor_screen_pos()[1]]);

            let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
            let pitch_set_changed = Slider::new(format!("##pitch_set{}", id), -89.0, 89.0)
                .display_format("pitch: %f")
                .build(ui, pitch);
            item_width_token.pop(ui);

            if pitch_x_clicked {
                None
            } else {
                Some(pitch_set_changed)
            }
        }
        None => {
            ui.set_cursor_screen_pos([yaw_pitch_menu_offset, ui.cursor_screen_pos()[1]]);

            let pitch_set_button_clicked = ui.button_with_size(
                format!("{}##pitch_set_button{}", "set pitch", id),
                [yaw_pitch_setter_width, 0.0],
            );

            if pitch_set_button_clicked {
                framebulk.pitch = Some(0.0);
            }

            Some(pitch_set_button_clicked)
        }
    };

    let edited_pitch = match edited_pitch {
        Some(edited_pitch) => edited_pitch,
        None => {
            framebulk.pitch = None;
            true
        }
    };

    edited_yaw || edited_pitch
}
