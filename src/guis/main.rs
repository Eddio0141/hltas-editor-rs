use std::{path::PathBuf};

use eframe::{
    egui::{self, menu, Color32},
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

struct Tab<'a> {
    title: String,
    path: Option<PathBuf>,
    hltas: HLTAS<'a>,
}

impl<'a> Tab<'a> {
    fn open_path(path: PathBuf) -> Self {
        Self {
            // TODO error check?
            // this is file so it should be
            title: path.file_name().unwrap().to_str().unwrap().to_owned(),
            path: Some(path),
            ..Default::default()
        }
    }

    fn new_file() -> Self {
        Self::default()
    }
}

impl<'a> Default for Tab<'a> {
    // TODO translation support
    fn default() -> Self {
        Self {
            title: "New file".to_owned(),
            path: None,
            hltas: Default::default(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainGUI<'a> {
    tabs: Vec<Tab<'a>>,
    // might have a chance to not have any tabs opened
    current_tab: Option<usize>,
}

impl<'a> MainGUI<'a> {
    pub fn new_file(&mut self) {
        self.tabs.push(Tab::new_file());
        self.current_tab = Some(self.tabs.len() - 1);
    }

    pub fn open_file(&mut self) {
        if let Ok(Some(pathbuf)) = FileDialog::new()
            .add_filter("HLTAS Files", &["hltas", "txt"])
            .show_open_single_file()
        {
            self.tabs.push(Tab::open_path(pathbuf));
        }
    }
}

impl<'a> Default for MainGUI<'a> {
    // first time opened will always show a new tab
    fn default() -> Self {
        Self {
            tabs: vec![Tab::default()],
            current_tab: Some(0),
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

            ui.separator();

            // tabs
            let mut stale_tabs: Vec<usize> = Vec::new();
            ui.horizontal(|ui| {
                for (index, tab) in self.tabs.iter().enumerate() {
                    // tab design
                    ui.group(|ui| {
                        ui.label(&tab.title);
                        let close_button = egui::Button::new("x")
                            .small()
                            .text_color(Color32::from_rgb(255, 0, 0));

                        if ui.add(close_button).clicked() {
                            // mark as stale
                            stale_tabs.push(index);
                        }
                    });
                }
            });

            stale_tabs.reverse();

            // remove stale tabs
            for index in stale_tabs {
                self.tabs.remove(index);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.text_edit_multiline(&mut self.raw_content);
        });
    }

    // TODO show current tab file
    fn name(&self) -> &str {
        "HLTAS Editor"
    }
}
