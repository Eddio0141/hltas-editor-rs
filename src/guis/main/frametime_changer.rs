use eframe::egui::{Color32, DragValue, Label, Ui, widgets};

use crate::helpers::hltas::{fps, frametime};

// TODO options for dragvalue
pub fn frametime_changer(frametime: &mut String, ui: &mut Ui) {
    if let Ok(mut frametime_f32) = frametime.parse::<f32>() {
        let frametime_prefix = String::from("frametime: ");
        let fps_prefix = String::from("fps: ");

        // TODO options for bg colors for fps presets
        let color_0ms = Color32::from_rgb(173, 216, 230);

        // TODO option or fps.rs?
        if frametime::is_0ms(frametime) {
            let bg_color = ui.visuals().code_bg_color;
            ui.add(
                Label::new(format!("{} 0ms", &frametime_prefix))
                    .background_color(bg_color)
                    .text_color(color_0ms),
            );
            ui.add(
                Label::new(format!("{} 0ms", &fps_prefix))
                    .background_color(bg_color)
                    .text_color(color_0ms),
            );
        } else {
            let changed_frametime = ui
                .add(
                    DragValue::new(&mut frametime_f32)
                        .speed(frametime::MAX_STRAFE / 4.0)
                        .clamp_range(frametime::MIN..=frametime::MAX_STRAFE)
                        .prefix(&frametime_prefix),
                )
                .changed();

            if changed_frametime {
                *frametime = frametime_f32.to_string();
            }

            let mut fps = 1.0 / frametime_f32;
            let changed_fps = ui
                .add(
                    DragValue::new(&mut fps)
                        .speed(0.2)
                        // TODO option or fps.rs?
                        .clamp_range(fps::MIN..=fps::MAX_STRAFE)
                        .prefix(&fps_prefix),
                )
                .changed();

            // only allows one to be changed at a time
            if changed_fps && !changed_frametime {
                *frametime = (1.0 / fps).to_string();
            }
        }

        // TODO option?
        // TODO customizable fps buttons?
        let frametime_0ms_button = widgets::Button::new("0ms frame").text_color(color_0ms);

        if ui.add(frametime_0ms_button).clicked() {
            // TODO use 0ms settings
            *frametime = "0.0000000001".to_string();
        }
    }
}
