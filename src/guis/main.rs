use std::{collections::VecDeque, fs, path::PathBuf};

use crate::helpers::locale::locale_lang::LocaleLang;
use crate::helpers::widget_stuff::menu_button::MenuButton;
use crate::helpers::{egui::memory::popup_state::PopupStateMemory, hltas::hltas_to_str};
use crate::widgets::menu::top_bottom_panel::tab::HLTASFileTab;
use eframe::egui::{Button, Window};
use eframe::{
    egui::{self, menu, Color32, FontDefinitions, FontFamily, Key, Label, Modifiers, Sense},
    epi,
};
use fluent_templates::Loader;
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
        self.tabs.push(HLTASFileTab::new_file(&self.locale_lang.get_lang()));
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
                fs::write(&path, &tab.raw_content)?;
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
            self.tabs.push(HLTASFileTab::new_file(&self.locale_lang.get_lang()));
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
                        // let mut recent_is_hovered = false;
                        // ui.horizontal(|ui| {
                        //     let recent_button = egui::Label::new("Recent >").sense(Sense::click().union(Sense::hover()));
                        //     // TODO make it stick to the right automatically
                        //     // ui.style_mut().spacing.item_spacing.x = 100.0;

                        //     if ui.add(recent_button).hovered() {
                        //         recent_is_hovered = true;
                        //     }

                        //     ui.label(">").ctx.pos;
                        // });

                        // TODO make this into its own widget
                        let recent_widget_pos = ui.min_rect().right_bottom();
                        let recent_widget = egui::Label::new(
                            crate::LOCALES.lookup(&self.locale_lang.get_lang(), "recent-files"),
                        )
                        .sense(Sense::hover());
                        let recent_button_response = ui.add(recent_widget);
                        let recent_popup_id = ui.make_persistent_id("recent_popup_id");
                        let mut make_recent_popup_window = false;
                        let mut recent_popup_coord = PopupStateMemory::none();

                        // check memory if the popup window is enabled

                        // not sure why can't i just use this for the if statement combination
                        // TODO better way to store with struct?
                        ui.memory()
                            .id_data_temp
                            .get_or_insert_with(recent_popup_id, || PopupStateMemory::none());

                        if let Some(pop_up_coords) = ui
                            .memory()
                            .id_data_temp
                            .get_mut::<PopupStateMemory>(&recent_popup_id)
                        {
                            if recent_button_response.hovered() {
                                // TODO set backup pos?
                                if let None = &pop_up_coords.0 {
                                    *pop_up_coords = PopupStateMemory {
                                        0: Some(recent_widget_pos),
                                    };
                                }
                            }
                            // *is_popped_up = recent_is_hovered;

                            if let Some(_) = pop_up_coords.0 {
                                // retarded solution yes
                                // TODO think of a cleaner way to do this
                                make_recent_popup_window = true;
                                recent_popup_coord = pop_up_coords.clone();
                            }
                        }

                        // TODO also think of a cleaner way to do this
                        let mut delete_recent_popup_window = false;
                        if make_recent_popup_window {
                            let show_pos = recent_popup_coord.0.unwrap();

                            if self.recent_paths.len() > 0 {
                                // TODO make the layer topmost with whatever method works
                                Window::new("recent_files_display")
                                    .title_bar(false)
                                    .auto_sized()
                                    .fixed_pos(show_pos)
                                    .show(ctx, |ui| {
                                        // TODO safety I guess, look into this issue a bit more
                                        let mut clicked_path: Option<PathBuf> = None;
                                        for recent_path in &self.recent_paths {
                                            if let Some(path_str) = recent_path.as_os_str().to_str()
                                            {
                                                let recent_path_button =
                                                    Button::new(path_str).frame(false);

                                                if ui.add(recent_path_button).clicked() {
                                                    clicked_path = Some(recent_path.to_owned());
                                                    break;
                                                }
                                            }
                                        }
                                        if let Some(clicked_path) = clicked_path {
                                            self.open_file(&clicked_path);
                                        }

                                        // BUG add delay for hover to be deleting popup or this will vanish instantly
                                        if ui.input().pointer.any_click() || (!recent_button_response.hovered() && !ui.ui_contains_pointer()) {
                                            delete_recent_popup_window = true;
                                        }
                                    });
                            }
                        }

                        if delete_recent_popup_window {
                            if let Some(pop_up_state) = ui
                                .memory()
                                .id_data_temp
                                .get_mut::<PopupStateMemory>(&recent_popup_id)
                            {
                                *pop_up_state = PopupStateMemory::none();
                            }
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
                                let current_tab_raw = &mut self.tabs[current_index].raw_content;
                                // TODO all error handling here
                                if let Ok(mut hltas) = HLTAS::from_str(&current_tab_raw) {
                                    cleaners::no_dupe_framebulks(&mut hltas);
                                    *current_tab_raw = hltas_to_str(&hltas);
                                }
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
            let mut stale_tabs: Vec<usize> = Vec::new();
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
            // ui.text_edit_multiline(&mut self.raw_content);
            // accept file drops
            for file in &ui.input().raw.dropped_files {
                if let Some(path) = &file.path {
                    self.open_file(path);
                }
            }
            // let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            //     let mut layout_job: egui::text::LayoutJob = my_memoized_highlighter(string);
            //     layout_job.wrap_width = wrap_width;
            //     ui.fonts().layout_job(layout_job)
            // };
            // ui.add(egui::TextEdit::multiline(&mut my_code).layouter(&mut layouter));

            // TODO show line count

            egui::ScrollArea::both().show(ui, |ui| {
                if let Some(current_tab_index) = self.current_tab_index {
                    let current_tab = &mut self.tabs[current_tab_index];
                    let tab_content_changed = ui
                        .add(
                            egui::TextEdit::multiline(&mut current_tab.raw_content)
                                .text_style(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_rows(1)
                                .lock_focus(true)
                                .desired_width(f32::INFINITY), // .layouter(&mut layouter)
                        )
                        .changed();

                    if tab_content_changed {
                        current_tab.got_modified = true;
                    }
                }
            });
        });

        // self.set_current_tab_title();
    }

    // TODO show current tab file
    fn name(&self) -> &str {
        // println!("title is {}", &self.title);
        &self.title
    }
}
