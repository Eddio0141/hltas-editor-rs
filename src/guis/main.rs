use std::{collections::VecDeque, fs, path::PathBuf};

use eframe::{
    egui::{
        self, menu, Color32, CtxRef, FontDefinitions, FontFamily, Key, Label, Modifiers, Pos2,
        Sense, Ui,
    },
    epi,
};
use fluent_templates::{LanguageIdentifier, Loader};
use hltas::HLTAS;
use hltas_cleaner::cleaners;
use locale_config::Locale;
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn hltas_to_str(hltas: &HLTAS) -> String {
    let mut file_u8: Vec<u8> = Vec::new();
    hltas.to_writer(&mut file_u8).unwrap();

    // always has to work
    if let Ok(content) = String::from_utf8(file_u8) {
        return content;
    }
    String::new()
}

fn default_lang() -> LanguageIdentifier {
    "en-US".parse::<LanguageIdentifier>().unwrap()
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct Tab {
    title: String,
    path: Option<PathBuf>,
    raw_content: String,
    got_modified: bool,
}

impl<'a> Tab {
    fn open_path(path: &PathBuf, file_content: &'a str) -> Result<Self, hltas::read::Error<'a>> {
        match HLTAS::from_str(&file_content) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }

        Ok(Self {
            // TODO error check?
            // this is file so it should be
            title: path.file_name().unwrap().to_str().unwrap().to_owned(),
            path: Some(path.clone()),
            raw_content: file_content.to_string(),
            // ..Default::default()
            got_modified: false,
        })
    }

    fn title_from_path(path: &PathBuf, lang: &LanguageIdentifier) -> String {
        if let Some(os_str) = path.file_name() {
            if let Some(str) = os_str.to_str() {
                return str.to_owned();
            }
        }
        Tab::default_title(lang).to_owned()
    }

    // BUG fix language change for title (opt out serialization for the titles?)
    fn default_title(lang: &LanguageIdentifier) -> String {
        crate::LOCALES.lookup(lang, "new-file-title")
    }

    fn new_file(lang: &LanguageIdentifier) -> Self {
        // TODO maybe make the language global?
        Self {
            title: Self::default_title(lang).to_string(),
            path: None,
            raw_content: hltas_to_str(&HLTAS::default()),
            got_modified: false,
        }
        // Self::default()
    }
}

// impl Default for Tab {
//     fn default() -> Self {
//         Self {
//             title: Tab::default_title().to_owned(),
//             path: None,
//             raw_content: hltas_to_str(&HLTAS::default()),
//             got_modified: false,
//         }
//     }
// }

fn key_to_string(key: &Key) -> &'static str {
    match key {
        Key::ArrowDown => "Arrow down",
        Key::ArrowLeft => "Arrow left",
        Key::ArrowRight => "Arrow right",
        Key::ArrowUp => "Arrow up",
        Key::Escape => "Escape",
        Key::Tab => "Tab",
        Key::Backspace => "Backspace",
        Key::Enter => "Enter",
        Key::Space => "Space",
        Key::Insert => "Insert",
        Key::Delete => "Delete",
        Key::Home => "Home",
        Key::End => "End",
        Key::PageUp => "Pageup",
        Key::PageDown => "Pagedown",
        Key::Num0 => "Num0",
        Key::Num1 => "Num1",
        Key::Num2 => "Num2",
        Key::Num3 => "Num3",
        Key::Num4 => "Num4",
        Key::Num5 => "Num5",
        Key::Num6 => "Num6",
        Key::Num7 => "Num7",
        Key::Num8 => "Num8",
        Key::Num9 => "Num9",
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
    }
}

// TODO key conflict check
struct MenuButton<T>
where
    T: FnMut(&mut MainGUI) -> (),
{
    shortcut: Option<(Key, Modifiers)>,
    pub name: String,
    // TODO better way to call in on_click?
    on_click: T,
}

impl<T> MenuButton<T>
where
    T: FnMut(&mut MainGUI) -> (),
{
    fn new(shortcut: Option<(Key, Modifiers)>, mut name: String, on_click: T) -> Self {
        if let Some(key_press) = shortcut {
            let key = &key_press.0;
            let modifiers = &key_press.1;

            let mut name_str_separated: Vec<String> = Vec::new();
            if modifiers.ctrl {
                name_str_separated.push("Ctrl".to_string());
            }
            if modifiers.alt {
                name_str_separated.push("Alt".to_string());
            }
            if modifiers.shift {
                name_str_separated.push("Shift".to_string());
            }
            // if modifiers.command
            if modifiers.mac_cmd {
                name_str_separated.push("⌘".to_string());
            }
            name_str_separated.push(key_to_string(key).to_string());

            name += &("      ".to_string() + &name_str_separated.join("+"));
        }

        Self {
            shortcut,
            name,
            on_click,
        }
    }

    fn key_check(&mut self, ctx: &CtxRef, main_gui: &mut MainGUI) {
        if let Some(key_modifiers) = &self.shortcut {
            let key = key_modifiers.0;
            let modifiers = key_modifiers.1;
            let input = ctx.input();

            if input.modifiers == modifiers && input.key_pressed(key) {
                (self.on_click)(main_gui);
            }
        }
    }

    fn create_button(&mut self, ui: &mut Ui, main_gui: &mut MainGUI) {
        if ui.button(&self.name).clicked() {
            (self.on_click)(main_gui);
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct LocaleLang {
    #[cfg_attr(feature = "persistence", serde(skip))]
    lang: Option<LanguageIdentifier>,
    // only used for serialization. makes sure it syncs with lang
    lang_str: Option<String>,
}

impl LocaleLang {
    pub fn new(lang: Option<LanguageIdentifier>) -> Self {
        let lang_str = match &lang {
            Some(some) => Some(some.to_string()),
            None => None,
        };

        Self { lang, lang_str }
    }

    pub fn get_lang(&mut self) -> LanguageIdentifier {
        // deserialization check
        if self.lang_str.is_some() && self.lang.is_none() {
            // got checked, lang_str is some
            let lang = match self
                .lang_str
                .to_owned()
                .unwrap()
                .parse::<LanguageIdentifier>()
            {
                Ok(lang) => lang,
                Err(_) => default_lang(),
            };

            self.lang = Some(lang);
        }

        match &self.lang {
            Some(lang) => lang.to_owned(),
            // shouldn't error
            None => Locale::current()
                .to_string()
                .parse()
                .unwrap_or_else(|_| default_lang()),
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct MainGUI {
    tabs: Vec<Tab>,
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
        self.tabs.push(Tab::new_file(&self.locale_lang.get_lang()));
        self.current_tab_index = Some(self.tabs.len() - 1);
    }

    pub fn open_file_by_dialog(&mut self) {
        if let Ok(Some(pathbuf)) = FileDialog::new()
            .add_filter("HLTAS Files", &["hltas", "txt"])
            .add_filter("Any", &["*"])
            .show_open_single_file()
        {
            self.open_file(pathbuf);
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

    pub fn open_file(&mut self, path: PathBuf) {
        if let Ok(file_content) = fs::read_to_string(&path) {
            match Tab::open_path(&path, &file_content) {
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
                    tab.title = Tab::title_from_path(&path, &self.locale_lang.get_lang());
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
            tabs: vec![Tab::new_file(&locale_lang.get_lang())],
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
            self.tabs.push(Tab::new_file(&self.locale_lang.get_lang()));
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
                let file_menu_pos = ui.spacing().item_spacing.x;
                menu::menu(
                    ui,
                    crate::LOCALES.lookup(&self.locale_lang.get_lang(), "file-menu"),
                    |ui| {
                        ui.set_width(200.0);

                        new_file.create_button(ui, self);
                        open_file.create_button(ui, self);
                        save_file.create_button(ui, self);
                        close_file.create_button(ui, self);

                        // HACK
                        // no side popup? oh well I guess I'll do the weird hack solution
                        // egui::popup::popup_below_widget(
                        //     ui,
                        //     recent_popup_id,
                        //     &recent_button_response,
                        //     |ui| {
                        //         // ui.set_min_width(200.0);
                        //         // ui.label("yoo!");
                        //     },
                        // );

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

                        let recent_button = egui::Label::new(
                            crate::LOCALES.lookup(&self.locale_lang.get_lang(), "recent-files"),
                        )
                        .sense(Sense::click().union(Sense::hover()));
                        let recent_button_response = ui.add(recent_button);
                        let recent_popup_id = ui.make_persistent_id("recent_popup_id");
                        let mut make_recent_popup_window = false;

                        // check memory if the popup window is enabled

                        // not sure why can't i just use this for the if statement combination
                        ui.memory()
                            .id_data_temp
                            .get_or_insert_with(recent_popup_id, || false);

                        if let Some(is_popped_up) =
                            ui.memory().id_data_temp.get_mut::<bool>(&recent_popup_id)
                        {
                            if recent_button_response.hovered() {
                                *is_popped_up = true;
                            }
                            // *is_popped_up = recent_is_hovered;

                            if *is_popped_up {
                                // retarded solution yes
                                // TODO think of a cleaner way to do this
                                make_recent_popup_window = true;
                            }
                        }

                        // TODO also think of a cleaner way to do this
                        let mut delete_recent_popup_window = false;
                        if make_recent_popup_window {
                            // HACK fix y pos
                            // HACK figure out x pos better
                            let window_pos =
                                Pos2::new(file_menu_pos + ui.available_width() + 4.0, 80.0);

                            if self.recent_paths.len() > 0 {
                                egui::Window::new("recent_files_window")
                                    .title_bar(false)
                                    .auto_sized()
                                    .fixed_pos(window_pos)
                                    .show(ctx, |ui| {
                                        // show recents
                                        for recent_path in self.recent_paths.iter() {
                                            if let Some(path_str) = recent_path.as_os_str().to_str()
                                            {
                                                ui.label(path_str);
                                            }
                                        }

                                        if ui.input().pointer.any_click() {
                                            delete_recent_popup_window = true;
                                        }
                                    });
                            }
                        }

                        if delete_recent_popup_window {
                            if let Some(is_popped_up) =
                                ui.memory().id_data_temp.get_mut::<bool>(&recent_popup_id)
                            {
                                *is_popped_up = false;
                            }
                        }
                    },
                );

                menu::menu(
                    ui,
                    crate::LOCALES.lookup(&self.locale_lang.get_lang(), "tools-menu"),
                    |ui| {
                        if ui.button(crate::LOCALES.lookup(&self.locale_lang.get_lang(), "hltas-cleaner")).clicked() {
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
                        if ui.button(crate::LOCALES.lookup(&self.locale_lang.get_lang(), "toggle-graphics-editor")).clicked() {
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
                    self.open_file(path.to_owned());
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
