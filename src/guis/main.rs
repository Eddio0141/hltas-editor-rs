use std::{fs, path::PathBuf};

use eframe::{
    egui::{self, menu, Color32, Label, Sense},
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

struct Tab {
    title: String,
    path: Option<PathBuf>,
    raw_content: String,
}

impl Tab {
    fn open_path(path: PathBuf) -> Result<Self, String> {
        if let Ok(file_content) = fs::read_to_string(&path) {
            match HLTAS::from_str(&file_content) {
                Ok(_) => {}
                // TODO better error handling
                Err(_) => {
                    return Err("Error, can't open the file as hltas file".to_owned());
                }
            }

            Ok(Self {
                // TODO error check?
                // this is file so it should be
                title: path.file_name().unwrap().to_str().unwrap().to_owned(),
                path: Some(path.clone()),
                raw_content: file_content,
            })
        } else {
            // TODO better error
            Err("Error, can't open the file".to_owned())
        }
    }

    fn new_file() -> Self {
        Self::default()
    }
}

impl Default for Tab {
    // TODO translation support
    fn default() -> Self {
        Self {
            title: "New file".to_owned(),
            path: None,
            raw_content: hltas_to_str(&HLTAS::default()),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainGUI {
    tabs: Vec<Tab>,
    // might have a chance to not have any tabs opened
    // TODO use direct reference
    current_tab_index: Option<usize>,
}

impl MainGUI {
    pub fn new_file(&mut self) {
        self.tabs.push(Tab::new_file());
        self.current_tab_index = Some(self.tabs.len() - 1);
    }

    pub fn open_file(&mut self) {
        if let Ok(Some(pathbuf)) = FileDialog::new()
            .add_filter("HLTAS Files", &["hltas", "txt"])
            .show_open_single_file()
        {
            // TODO error handling here
            if let Ok(tab) = Tab::open_path(pathbuf) {
                self.tabs.push(tab);
                self.current_tab_index = Some(self.tabs.len() - 1);
            }
        }
    }
}

impl Default for MainGUI {
    // first time opened will always show a new tab
    fn default() -> Self {
        Self {
            tabs: vec![Tab::default()],
            current_tab_index: Some(0),
        }
    }
}

impl epi::App for MainGUI {
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
                let mut new_index: Option<usize> = None;
                for (index, tab) in self.tabs.iter().enumerate() {
                    // tab design
                    ui.group(|ui| {
                        // if label is clicked, switch to that one
                        if ui
                            .add(Label::new(&tab.title).sense(Sense::click()))
                            .clicked()
                        {
                            // FIXME not sure why I can't do this
                            // self.current_tab_index = Some(index);
                            new_index = Some(index);
                        }

                        let close_button = egui::Button::new("x")
                            .small()
                            .text_color(Color32::from_rgb(255, 0, 0));

                        if ui.add(close_button).clicked() {
                            // mark as stale
                            stale_tabs.push(index);
                        }
                    });
                }

                if let Some(_) = new_index {
                    self.current_tab_index = new_index;
                }
            });

            stale_tabs.reverse();

            // remove stale tabs
            for index in stale_tabs {
                self.tabs.remove(index);
            }

            // fix index if its out of bounds
            if let Some(index) = self.current_tab_index {
                if self.tabs.len() == 0 {
                    self.current_tab_index = None;
                } else if index >= self.tabs.len() {
                    self.current_tab_index = Some(self.tabs.len() - 1);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(current_tab_index) = self.current_tab_index {
                let current_tab = &mut self.tabs[current_tab_index];
                ui.text_edit_multiline(&mut current_tab.raw_content);
            }
        });
    }

    // TODO show current tab file
    fn name(&self) -> &str {
        "HLTAS Editor"
    }
}
