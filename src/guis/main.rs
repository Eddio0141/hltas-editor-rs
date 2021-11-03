use eframe::{
    egui::{self, menu, Color32, Widget},
    epi,
};
use hltas::HLTAS;

struct TabWidget<'a> {
    title: String,
    path: Option<String>,
    hltas: HLTAS<'a>,

    index: usize,
}

impl<'a> Widget for &TabWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.label(&self.title);
            let close_button = egui::Button::new("x")
                .small()
                .text_color(Color32::from_rgb(255, 0, 0));

            if ui.add(close_button).clicked() {
                // TODO
                //self.tabs_vec.remove(self.index);
            }
        })
        .response
    }
}

impl<'a> TabWidget<'a> {
    fn open_path(path: Option<String>, index: usize) -> Self {
        // TODO
        Self {
            title: "".to_owned(),
            path,
            hltas: HLTAS::default(),
            index,
        }
    }

    fn new_file(index: usize) -> Self {
        // TODO, translation support
        Self {
            title: "New file".to_owned(),
            path: None,
            hltas: HLTAS::default(),
            index,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct MainGUI {
    tabs: Vec<TabWidget<'static>>,
    test: String,
}

impl MainGUI {
    pub fn new_file(&mut self) {
        self.tabs.push(TabWidget::new_file(self.tabs.len()));
    }

    pub fn close_file(&mut self, index: usize) {
        if let Some(_) = self.tabs.get(index) {
            self.tabs.remove(index);
            // TODO save handling
        }
    }
}

impl Default for MainGUI {
    fn default() -> Self {
        let mut gui = Self {
            tabs: Vec::new(),
            test: "\
s03----c--|------|------|0.001|90|-|13
----------|------|--u---|0.0000000001|225|-|11
----------|------|------|0.0000000001|0|-|1
s03----c--|------|------|0.001|90|-|7
----------|------|--u---|0.0000000001|225|-|11
----------|------|------|0.0000000001|0|-|1
s03----c--|------|------|0.001|90|-|12
s03----c--|------|------|0.001|180|-|1
----------|------|--u---|0.0000000001|225|-|20
----------|------|------|0.0000000001|0|-|1
s03----c--|------|------|0.001|180|-|13"
                .to_owned(),
        };
        gui.new_file();
        gui
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
                })
            });

            ui.separator();

            // tabs
            ui.horizontal(|ui| {
                for tab in &self.tabs {
                    ui.add(tab);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.test);
        });
    }

    fn name(&self) -> &str {
        "HLTAS Editor"
    }
}
