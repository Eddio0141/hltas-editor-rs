use eframe::egui::{Button, Color32, DragValue, Ui};

use crate::helpers::hltas::fps;

// TODO options for dragvalue
pub fn frametime_changer(frametime: &mut String, ui: &mut Ui) {
    if let Ok(mut frametime_f32) = frametime.parse::<f32>() {
        // TODO option or fps.rs?
        // TODO 0ms shows something different
        let changed_frametime = ui
            .add(
                DragValue::new(&mut frametime_f32)
                    .speed(0.001)
                    .clamp_range(0.001..=fps::MIN)
                    .prefix("frametime: "),
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
                    .clamp_range(1000.0..=4.0)
                    .prefix("fps: "),
            )
            .changed();

        // only allows one to be changed at a time
        if changed_fps && !changed_frametime {
            *frametime = (1.0 / fps).to_string();
        }

        // TODO option?
        // TODO customizable fps buttons?
        let frametime_0ms_button = Button::new("0ms frame").text_color(Color32::from_rgb(173, 216, 230));

        if ui.add(frametime_0ms_button).clicked() && (!changed_fps && !changed_frametime) {
            // TODO use 0ms settings
            *frametime = "0.0000000001".to_string();
        }
    }
}
