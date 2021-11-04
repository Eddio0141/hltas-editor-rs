use std::{borrow::BorrowMut, fs::{self, File}, io::Cursor, path::PathBuf};

use eframe::{
    egui::{self, menu, Button, Color32, Widget},
    epi,
};
use hltas::HLTAS;
use native_dialog::FileDialog;

fn hltas_to_str(hltas: &HLTAS) -> String {
    let mut file_u8: Vec<u8> = Vec::new();
    hltas.to_writer(&mut file_u8).unwrap();

    // always has to work
    if let Ok(content) = String::from_utf8(file_u8) {
        return content;
    }
    String::new()
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainGUI<'a> {
    current_file: Option<PathBuf>,
    hltas: HLTAS<'a>,
    raw_content: String ,
}

impl<'a> MainGUI<'a> {
    pub fn new_file(&mut self) {
        *self = MainGUI::default();
    }

    pub fn open_file(&'a mut self) {
        if let Ok(Some(pathbuf)) = FileDialog::new()
            .add_filter("HLTAS Files", &["hltas", "txt"])
            .show_open_single_file()
        {
            self.current_file = Some(pathbuf);

            if let Ok(hltas_file_str) = fs::read_to_string(self.current_file.as_ref().unwrap()) {
                self.raw_content = hltas_file_str.to_owned();
                match HLTAS::from_str(&self.raw_content) {
                    Ok(file) => self.hltas = file.to_owned(),
                    Err(_) => todo!(),
                }
                // if let Ok(hltas_file_content) = HLTAS::from_str(hltas_file_str) {
                //     self.hltas = hltas_file_content.clone();
                // }
            }
            // TODO, failed to open
        }
    }
}

impl<'a> Default for MainGUI<'a> {
    // first time opened will always show a new tab
    fn default() -> Self {
        Self {
            current_file: None,
            hltas: HLTAS::default(),
            raw_content: hltas_to_str(&HLTAS::default()),
        }
    }
}

impl<'a> epi::App for MainGUI<'a> {
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    // fn warm_up_enabled(&self) -> bool {
    //     false
    // }

    // fn save(&mut self, _storage: &mut dyn epi::Storage) {}

    // fn on_exit(&mut self) {}

    // fn auto_save_interval(&self) -> std::time::Duration {
    //     std::time::Duration::from_secs(30)
    // }

    // fn clear_color(&self) -> egui::Rgba {
    //     // NOTE: a bright gray makes the shadows of the windows look weird.
    //     // We use a bit of transparency so that if the user switches on the
    //     // `transparent()` option they get immediate results.
    //     egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    // }

    // fn persist_native_window(&self) -> bool {
    //     true
    // }

    // fn persist_egui_memory(&self) -> bool {
    //     true
    // }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                menu::menu(ui, "File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_file();
                    }
                    if ui.button("Open").clicked() {
                        self.open_file();
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.raw_content);
        });
    }

    fn name(&self) -> &str {
        "HLTAS Editor"
    }
}
