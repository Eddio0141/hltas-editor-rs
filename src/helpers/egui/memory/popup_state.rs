use eframe::egui::Pos2;

#[derive(Clone)]
pub struct PopupStateMemory(pub Option<Pos2>);

impl PopupStateMemory {
    pub fn none() -> Self {
        Self { 0: None }
    }
}
