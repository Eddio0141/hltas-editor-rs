use eframe::egui::{Button, Color32};

pub fn close_button() -> Button {
    Button::new("x").text_color(Color32::from_rgb(255, 0, 0))
}