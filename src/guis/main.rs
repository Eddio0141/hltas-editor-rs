use std::{collections::VecDeque, fs, path::PathBuf};

use crate::helpers::egui::button::close_button;
use crate::helpers::egui::containers::popup_to_widget_right;
use crate::helpers::hltas::{fps, hltas_to_str};
use crate::helpers::locale::locale_lang::LocaleLang;
use crate::helpers::widget_stuff::menu_button::MenuButton;
use crate::widgets::hltas::frametime_changer;
use crate::widgets::menu::top_bottom_panel::tab::HLTASFileTab;
use eframe::egui::{Button, CollapsingHeader, DragValue};
use eframe::{
    egui::{self, menu, FontDefinitions, FontFamily, Key, Label, Modifiers, Sense},
    epi,
};
use fluent_templates::Loader;
use hltas::types::{Line, Seeds};
use hltas::HLTAS;
use hltas_cleaner::cleaners;
use native_dialog::{FileDialog, MessageDialog, MessageType};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct MainGUI {
    tabs: Vec<HLTASFileTab>,
    // might have a chance to not have any tabs opened
    // TODO use direct reference
    current_tab_index: Option<usize>,
    title: String,
    // TODO option to change size
    recent_paths: VecDeque<PathBuf>,
    graphics_editor: bool,
    locale_lang: LocaleLang,
}

impl MainGUI {
    // TODO make it a field?
    pub const fn recent_path_max_size() -> usize {
        20
    }

    pub fn new_file(&mut self) {
        // TODO method to do this tab switching?
        self.tabs
            .push(HLTASFileTab::new_file(&self.locale_lang.get_lang()));
        self.current_tab_index = Some(self.tabs.len() - 1);
    }

    pub fn open_file_by_dialog(&mut self) {
        if let Ok(Some(pathbuf)) = FileDialog::new()
            .add_filter("HLTAS Files", &["hltas", "txt"])
            .add_filter("Any", &["*"])
            .show_open_single_file()
        {
            self.open_file(&pathbuf);
        }
    }

    fn add_recent_path(&mut self, path: &PathBuf) {
        let path_as_str = path.as_os_str().to_str();

        // dupe check, deletes dupe
        if let Some(dupe_index) = self.recent_paths.iter().position(|recent_path| {
            if let Some(recent_path_str) = recent_path.as_os_str().to_str() {
                if let Some(path_str) = path_as_str {
                    return recent_path_str == path_str;
                }
            }
            false
        }) {
            self.recent_paths.remove(dupe_index);
        }

        self.recent_paths.push_back(path.clone());

        if self.recent_paths.len() > Self::recent_path_max_size() {
            self.recent_paths.pop_front();
        }
    }

    pub fn open_file(&mut self, path: &PathBuf) {
        // check for dupe tab and switch to it if found
        let dupe_tab_index = self.tabs.iter().position(|tab| {
            if let Some(tab_path) = &tab.path {
                return tab_path.as_path() == path.as_path();
            }
            false
        });

        if let Some(dupe_tab_index) = dupe_tab_index {
            self.current_tab_index = Some(dupe_tab_index);
            return;
        }

        if let Ok(file_content) = fs::read_to_string(&path) {
            match HLTASFileTab::open_path(&path, &file_content) {
                Ok(tab) => {
                    self.tabs.push(tab);
                    self.current_tab_index = Some(self.tabs.len() - 1);

                    self.add_recent_path(&path);
                }
                Err(err) => {
                    MessageDialog::new()
                        .set_title("Error, Cannot parse as hltas file")
                        .set_text(&err.to_string())
                        .set_type(MessageType::Error)
                        .show_alert()
                        .ok();
                }
            }
        }
    }

    fn ask_hltas_save_location() -> Result<Option<PathBuf>, native_dialog::Error> {
        FileDialog::new()
            .add_filter("HLTAS Files", &["hltas"])
            .show_save_single_file()
    }

    pub fn save_current_tab(&mut self, warn_user: Option<String>) -> Result<(), std::io::Error> {
        if let Some(current_tab) = self.current_tab_index {
            if let Some(warning_msg) = warn_user {
                // pop up warning!
                let warning_user_selection = native_dialog::MessageDialog::new()
                    .set_title("Warning!")
                    .set_type(native_dialog::MessageType::Warning)
                    .set_text(&warning_msg)
                    .show_confirm();

                if let Ok(warning_user_selection) = warning_user_selection {
                    if !warning_user_selection {
                        return Ok(());
                    }
                }
            }

            let tab = &mut self.tabs[current_tab];
            let mut save_path: Option<PathBuf> = None;
            let mut new_file = false;
            println!("current tab index {:#?}", &self.current_tab_index);
            println!("current tab path {:#?}", &tab.path);
            if let Some(path) = &tab.path {
                save_path = Some(path.to_owned());
            } else {
                // no file, save as new file
                if let Ok(path) = Self::ask_hltas_save_location() {
                    save_path = path;
                    new_file = true;
                }
            }

            if let Some(path) = save_path {
                fs::write(&path, hltas_to_str(&tab.hltas))?;
                if new_file {
                    tab.title = HLTASFileTab::title_from_path(&path, &self.locale_lang.get_lang());
                }
            }
        }

        Ok(())
    }

    pub fn close_current_tab(&mut self) {
        if let Some(index) = self.current_tab_index {
            let current_tab = &self.tabs[index];

            if current_tab.got_modified {
                if let Ok(_) = self.save_current_tab(Some(
                    "Would you like to save the modified file?".to_string(),
                )) {
                    self.tabs.remove(index);
                }
                // else do nothing since we can't close the tab
            } else {
                self.tabs.remove(index);
            }
        }
    }

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }

        let current_tab = &self.tabs[index];

        if current_tab.got_modified {
            if let Ok(_) = self.save_current_tab(Some(
                "Would you like to save the modified file?".to_string(),
            )) {
                self.tabs.remove(index);
            }
            // else do nothing since we can't close the tab
        } else {
            self.tabs.remove(index);
        }
    }

    // pub fn set_current_tab_title(&mut self) {
    //     if let Some(index) = self.current_tab_index {
    //         // println!("current index {}", index);
    //         self.title = Self::default_title().to_string() + " - " + &self.tabs[index].title;
    //         return;
    //     } else {
    //         // println!("none");
    //         self.title = Self::default_title().to_string();
    //     }
    //     println!("title is {}", &self.title);
    // }

    fn default_title() -> &'static str {
        "HLTAS Editor"
    }
}

impl Default for MainGUI {
    // first time opened will always show a new tab
    fn default() -> Self {
        let mut locale_lang = LocaleLang::new(None);

        Self {
            tabs: vec![HLTASFileTab::new_file(&locale_lang.get_lang())],
            current_tab_index: Some(0),
            title: Self::default_title().to_string(),
            recent_paths: VecDeque::new(),
            graphics_editor: true,
            locale_lang,
        }
    }
}

// TODO separate into different functions to clean this up
impl epi::App for MainGUI {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        // always have 1 tab opened by default
        if self.tabs.len() == 0 {
            // self.tabs.push(Tab::default());
            self.tabs
                .push(HLTASFileTab::new_file(&self.locale_lang.get_lang()));
            self.current_tab_index = Some(0);
        }

        // TODO use system fonts and somehow match language
        let mut fonts = FontDefinitions::default();
        let msgothic_font = "msgothic";

        fonts.font_data.insert(
            msgothic_font.to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../../fonts/msgothic.ttc")),
        );
        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, msgothic_font.to_owned());

        ctx.set_fonts(fonts);

        // attempt to load files cause it could change content
        // TODO change this into a check if file changed, I still want to store state of edited hltas in the editor
        let mut stale_tabs = Vec::new();

        if self.tabs.len() > 0 {
            for (i, tab) in self.tabs.iter_mut().enumerate() {
                if let Some(path) = &tab.path {
                    match fs::read_to_string(&path) {
                        Ok(content) => match HLTAS::from_str(&content) {
                            Ok(hltas) => tab.hltas = hltas,
                            Err(_) => stale_tabs.push(i),
                        },
                        Err(_) => stale_tabs.push(i),
                    }
                }
            }
        }

        // TODO think of a better way to handle this
        stale_tabs.reverse();

        for stale_tab in stale_tabs {
            self.close_tab(stale_tab);
        }
    }

    // fn warm_up_enabled(&self) -> bool {
    //     false
    // }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

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
        // TODO even more clean up of this
        let mut new_file = MenuButton::new(
            Some((
                Key::N,
                Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            )),
            crate::LOCALES
                .lookup(&self.locale_lang.get_lang(), "new-file")
                .to_string(),
            |main_gui| main_gui.new_file(),
        );
        let mut open_file = MenuButton::new(
            Some((
                Key::O,
                Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            )),
            crate::LOCALES
                .lookup(&self.locale_lang.get_lang(), "open-file")
                .to_string(),
            |main_gui| main_gui.open_file_by_dialog(),
        );
        let mut save_file = MenuButton::new(
            Some((
                Key::S,
                Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            )),
            crate::LOCALES
                .lookup(&self.locale_lang.get_lang(), "save-file")
                .to_string(),
            // TODO error handle
            |main_gui| {
                main_gui.save_current_tab(None).ok();
            },
        );
        let mut close_file = MenuButton::new(
            Some((
                Key::W,
                Modifiers {
                    ctrl: true,
                    command: true,
                    ..Default::default()
                },
            )),
            crate::LOCALES
                .lookup(&self.locale_lang.get_lang(), "close-file")
                .to_string(),
            |main_gui| main_gui.close_current_tab(),
        );

        // menu input checks
        new_file.key_check(&ctx, self);
        open_file.key_check(&ctx, self);
        save_file.key_check(&ctx, self);
        close_file.key_check(&ctx, self);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                menu::menu(
                    ui,
                    crate::LOCALES.lookup(&self.locale_lang.get_lang(), "file-menu"),
                    |ui| {
                        ui.set_width(200.0);

                        new_file.create_button(ui, self);
                        open_file.create_button(ui, self);
                        save_file.create_button(ui, self);
                        close_file.create_button(ui, self);

                        // TODO make it look like | Recent       > |
                        let recent_popup_id = ui.make_persistent_id("recent_popup_id");
                        let recent_opener = egui::Label::new(
                            crate::LOCALES.lookup(&self.locale_lang.get_lang(), "recent-files"),
                        )
                        .sense(Sense::hover());
                        let recent_opener_response = ui.add(recent_opener);

                        if !ui.memory().is_popup_open(recent_popup_id)
                            && recent_opener_response.hovered()
                        {
                            ui.memory().open_popup(recent_popup_id);
                        }

                        let popup_hovered = popup_to_widget_right(
                            ui,
                            recent_popup_id,
                            &recent_opener_response,
                            |ui| {
                                let clicked_path = {
                                    let mut clicked_path = None;
                                    for recent_path in &self.recent_paths {
                                        if let Some(path_str) = recent_path.as_os_str().to_str() {
                                            let recent_path_button =
                                                Button::new(path_str).frame(false).wrap(false);

                                            if ui.add(recent_path_button).clicked() {
                                                clicked_path = Some(recent_path.to_owned());
                                                break;
                                            }
                                        }
                                    }
                                    clicked_path
                                };

                                if let Some(clicked_path) = clicked_path {
                                    self.open_file(&clicked_path);
                                }

                                return ui.rect_contains_pointer(ui.clip_rect());
                            },
                        );
                        let popup_hovered = {
                            if let Some(hovered) = popup_hovered {
                                hovered
                            } else {
                                false
                            }
                        };

                        if !recent_opener_response.hovered() && !popup_hovered {
                            ui.memory().close_popup();
                        }
                    },
                );

                menu::menu(
                    ui,
                    crate::LOCALES.lookup(&self.locale_lang.get_lang(), "tools-menu"),
                    |ui| {
                        if ui
                            .button(
                                crate::LOCALES
                                    .lookup(&self.locale_lang.get_lang(), "hltas-cleaner"),
                            )
                            .clicked()
                        {
                            // TODO show options
                            if let Some(current_index) = self.current_tab_index {
                                // TODO all error handling here
                                let current_hltas = &mut self.tabs[current_index].hltas;
                                cleaners::no_dupe_framebulks(current_hltas);
                            }
                        }
                    },
                );

                menu::menu(
                    ui,
                    crate::LOCALES.lookup(&self.locale_lang.get_lang(), "options-menu"),
                    |ui| {
                        if ui
                            .button(
                                crate::LOCALES
                                    .lookup(&self.locale_lang.get_lang(), "toggle-graphics-editor"),
                            )
                            .clicked()
                        {
                            self.graphics_editor = !self.graphics_editor;
                        }
                    },
                );
            });

            ui.separator();

            // tabs
            let mut stale_tab: Option<usize> = None;
            egui::ScrollArea::horizontal().show(ui, |ui| {
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

                            let close_button = close_button().small();

                            if ui.add(close_button).clicked() {
                                // mark as stale
                                stale_tab = Some(index);
                            }
                        });
                    }

                    if let Some(_) = new_index {
                        self.current_tab_index = new_index;
                    }
                });
            });

            // remove stale tab
            if let Some(stale_tab) = stale_tab {
                self.tabs.remove(stale_tab);
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
            // ui.text_edit_multiline(&mut self.raw_content);
            // accept file drops
            for file in &ui.input().raw.dropped_files {
                if let Some(path) = &file.path {
                    self.open_file(path);
                }
            }

            if let Some(current_tab_index) = self.current_tab_index {
                let current_tab = &mut self.tabs[current_tab_index];

                if self.graphics_editor {
                    egui::ScrollArea::both().show(ui, |ui| {
                        // TODO translation?
                        let hltas = &mut current_tab.hltas;

                        CollapsingHeader::new("properties")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("frametime0ms");
                                    let set_frametime_button =
                                        match &mut hltas.properties.frametime_0ms {
                                            Some(frametime) => match frametime.parse::<f32>() {
                                                Ok(mut frametime) => {
                                                    ui.add(
                                                        DragValue::new(&mut frametime)
                                                            .speed(fps::MAX)
                                                            .clamp_range(fps::MAX..=fps::MIN),
                                                    );
                                                    hltas.properties.frametime_0ms =
                                                        Some(frametime.to_string());
                                                    if ui.add(close_button().small()).clicked() {
                                                        hltas.properties.frametime_0ms = None;
                                                    }
                                                    false
                                                }
                                                Err(_) => true,
                                            },
                                            None => true,
                                        };

                                    if set_frametime_button {
                                        if ui.button("set frametime0ms").clicked() {
                                            // TODO implement settings to change this
                                            hltas.properties.frametime_0ms =
                                                Some("0.0000000001".to_string());
                                        }
                                    }

                                    ui.shrink_width_to_current();
                                });

                                ui.horizontal(|ui| {
                                    ui.label("seeds");
                                    let create_seed_button = match &mut hltas.properties.seeds {
                                        Some(seeds) => {
                                            let shared_rng = &mut seeds.shared;
                                            let nonshared_rng = &mut seeds.non_shared;

                                            ui.add(DragValue::new(shared_rng).speed(0.2));
                                            ui.add(DragValue::new(nonshared_rng).speed(0.2));
                                            if ui.add(close_button().small()).clicked() {
                                                hltas.properties.seeds = None;
                                            }
                                            false
                                        }
                                        None => true,
                                    };

                                    if create_seed_button {
                                        if ui.button("set shared non-shared rng").clicked() {
                                            hltas.properties.seeds = Some(Seeds {
                                                shared: 0,
                                                non_shared: 0,
                                            });
                                        }
                                    }

                                    ui.shrink_width_to_current();
                                });

                                // TODO remove me
                                ui.horizontal(|ui| {
                                    ui.label(hltas.lines.len());
                                    if hltas.lines.len() > 15 {
                                        if let Line::FrameBulk(framebulk) = &mut hltas.lines[15] {
                                            ui.label("shud be working");
                                            frametime_changer(&mut framebulk.frame_time, ui);
                                        }
                                    }
                                });
                            });
                    });
                } else {
                    // let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    //     let mut layout_job: egui::text::LayoutJob = my_memoized_highlighter(string);
                    //     layout_job.wrap_width = wrap_width;
                    //     ui.fonts().layout_job(layout_job)
                    // };
                    // ui.add(egui::TextEdit::multiline(&mut my_code).layouter(&mut layouter));

                    // TODO show line count
                    // egui::ScrollArea::both().show(ui, |ui| {
                    //     // HACK find a better method
                    //     let mut raw_hltas = current_tab.get_raw_content().to_owned();
                    //     let tab_content_changed = ui
                    //         .add(
                    //             egui::TextEdit::multiline(&mut raw_hltas)
                    //                 .text_style(egui::TextStyle::Monospace)
                    //                 .code_editor()
                    //                 .desired_rows(1)
                    //                 .lock_focus(true)
                    //                 .desired_width(f32::INFINITY), // .layouter(&mut layouter)
                    //         )
                    //         .changed();

                    //     if tab_content_changed {
                    //         if let Ok(hltas) = HLTAS::from_str(&raw_hltas) {
                    //             current_tab.set_hltas(hltas);
                    //         }
                    //     }
                    //     if tab_content_changed {
                    //         current_tab.got_modified = true;
                    //     }
                    // });
                }
            }
        });

        // self.set_current_tab_title();
    }

    // TODO show current tab file
    fn name(&self) -> &str {
        // println!("title is {}", &self.title);
        &self.title
    }
}
