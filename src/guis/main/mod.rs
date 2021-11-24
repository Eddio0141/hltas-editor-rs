mod frametime_changer;
mod graphics_editor;
mod selectable_hltas_button;
mod strafe_key_selector;
mod tab;
mod text_editor;

use std::path::Path;
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::helpers::hltas::hltas_to_str;
use crate::helpers::locale::locale_lang::LocaleLang;
use fluent_templates::Loader;

use hltas::HLTAS;
use hltas_cleaner::cleaners;
use imgui::{Key, MenuItem, Ui, Window};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use winit::event::VirtualKeyCode;

use self::graphics_editor::show_graphics_editor;
use self::tab::HLTASFileTab;
use self::text_editor::show_text_editor;

pub struct MainGUI {
    test: bool,
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

    fn add_recent_path(&mut self, path: &Path) {
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

        self.recent_paths.push_back(path.to_owned());

        if self.recent_paths.len() > Self::recent_path_max_size() {
            self.recent_paths.pop_front();
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        // check for dupe tab and switch to it if found
        let dupe_tab_index = self.tabs.iter().position(|tab| {
            if let Some(tab_path) = &tab.path {
                return tab_path.as_path() == path;
            }
            false
        });

        if let Some(dupe_tab_index) = dupe_tab_index {
            self.current_tab_index = Some(dupe_tab_index);
            return;
        }

        if let Ok(file_content) = fs::read_to_string(&path) {
            match HLTASFileTab::open_path(path, &file_content) {
                Ok(tab) => {
                    self.tabs.push(tab);
                    self.current_tab_index = Some(self.tabs.len() - 1);

                    self.add_recent_path(path);
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
            if let Some(path) = &tab.path {
                // save_path = Some(path.to_owned());
                fs::write(path, hltas_to_str(&tab.hltas))?;
            } else {
                // no file, save as new file
                if let Ok(path) = Self::ask_hltas_save_location() {
                    if let Some(path) = path {
                        fs::write(&path, hltas_to_str(&tab.hltas))?;
                        tab.title =
                            HLTASFileTab::title_from_path(&path, &self.locale_lang.get_lang());
                    }
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
            test: false,
        }
    }
}

impl MainGUI {
    //     // always have 1 tab opened by default
    //     if self.tabs.len() == 0 {
    //         // self.tabs.push(Tab::default());
    //         self.tabs
    //             .push(HLTASFileTab::new_file(&self.locale_lang.get_lang()));
    //         self.current_tab_index = Some(0);
    //     }

    //     // TODO use system fonts and somehow match language
    //     let mut fonts = FontDefinitions::default();
    //     let msgothic_font = String::from("msgothic");

    //     fonts.font_data.insert(
    //         msgothic_font.to_owned(),
    //         std::borrow::Cow::Borrowed(include_bytes!("../../../fonts/msgothic.ttc")),
    //     );
    //     fonts
    //         .fonts_for_family
    //         .get_mut(&FontFamily::Proportional)
    //         .unwrap()
    //         .insert(0, msgothic_font);

    //     ctx.set_fonts(fonts);

    //     // attempt to load files cause it could change content
    //     // TODO change this into a check if file changed, I still want to store state of edited hltas in the editor
    //     let mut stale_tabs = Vec::new();

    //     if self.tabs.len() > 0 {
    //         for (i, tab) in self.tabs.iter_mut().enumerate() {
    //             if let Some(path) = &tab.path {
    //                 match fs::read_to_string(&path) {
    //                     Ok(content) => match HLTAS::from_str(&content) {
    //                         Ok(hltas) => tab.hltas = hltas,
    //                         Err(_) => stale_tabs.push(i),
    //                     },
    //                     Err(_) => stale_tabs.push(i),
    //                 }
    //             }
    //         }
    //     }

    //     // TODO think of a better way to handle this
    //     stale_tabs.reverse();

    //     for stale_tab in stale_tabs {
    //         self.close_tab(stale_tab);
    //     }
    // }

    pub fn show(&mut self, _run: &mut bool, ui: &mut Ui) {
        if let Some(main_menu) = ui.begin_main_menu_bar() {
            ui.menu("File", || {
                if MenuItem::new("New").shortcut("Ctrl+O").build(ui) || (ui.io().keys_down[VirtualKeyCode::O as usize] && ui.io().key_ctrl ){
                    self.open_file_by_dialog();
                }
            });


            main_menu.end();
        }

        // TODO even more clean up of this
        // let mut new_file = MenuButton::new(
        //     Some((
        //         Key::N,
        //         Modifiers {
        //             ctrl: true,
        //             command: true,
        //             ..Default::default()
        //         },
        //     )),
        //     "new-file",
        //     self.locale_lang.get_lang(),
        //     |main_gui| main_gui.new_file(),
        // );
        // let mut open_file = MenuButton::new(
        //     Some((
        //         Key::O,
        //         Modifiers {
        //             ctrl: true,
        //             command: true,
        //             ..Default::default()
        //         },
        //     )),
        //     "open-file",
        //     self.locale_lang.get_lang(),
        //     |main_gui| main_gui.open_file_by_dialog(),
        // );
        // let mut save_file = MenuButton::new(
        //     Some((
        //         Key::S,
        //         Modifiers {
        //             ctrl: true,
        //             command: true,
        //             ..Default::default()
        //         },
        //     )),
        //     "save-file",
        //     self.locale_lang.get_lang(),
        //     // TODO error handle
        //     |main_gui| {
        //         main_gui.save_current_tab(None).ok();
        //     },
        // );
        // let mut close_file = MenuButton::new(
        //     Some((
        //         Key::W,
        //         Modifiers {
        //             ctrl: true,
        //             command: true,
        //             ..Default::default()
        //         },
        //     )),
        //     "close-file",
        //     self.locale_lang.get_lang(),
        //     |main_gui| main_gui.close_current_tab(),
        // );
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     menu::bar(ui, |ui| {
        //         menu::menu(
        //             ui,
        //             crate::LOCALES.lookup(&self.locale_lang.get_lang(), "file-menu"),
        //             |ui| {
        //                 ui.set_width(200.0);

        //                 // TODO make it look like | Recent       > |
        //                 let recent_popup_id = ui.make_persistent_id("recent_popup_id");
        //                 let recent_opener = egui::Label::new(
        //                     crate::LOCALES.lookup(&self.locale_lang.get_lang(), "recent-files"),
        //                 )
        //                 .sense(Sense::hover());
        //                 let recent_opener_response = ui.add(recent_opener);

        //                 if !ui.memory().is_popup_open(recent_popup_id)
        //                     && recent_opener_response.hovered()
        //                 {
        //                     ui.memory().open_popup(recent_popup_id);
        //                 }

        //                 let popup_hovered = popup_to_widget_right(
        //                     ui,
        //                     recent_popup_id,
        //                     &recent_opener_response,
        //                     |ui| {
        //                         let clicked_path = {
        //                             let mut clicked_path = None;
        //                             for recent_path in &self.recent_paths {
        //                                 if let Some(path_str) = recent_path.as_os_str().to_str() {
        //                                     let recent_path_button =
        //                                         Button::new(path_str).frame(false).wrap(false);

        //                                     if ui.add(recent_path_button).clicked() {
        //                                         clicked_path = Some(recent_path.to_owned());
        //                                         break;
        //                                     }
        //                                 }
        //                             }
        //                             clicked_path
        //                         };

        //                         if let Some(clicked_path) = clicked_path {
        //                             self.open_file(&clicked_path);
        //                         }

        //                         return ui.rect_contains_pointer(ui.clip_rect());
        //                     },
        //                 );
        //                 let popup_hovered = {
        //                     if let Some(hovered) = popup_hovered {
        //                         hovered
        //                     } else {
        //                         false
        //                     }
        //                 };

        //                 if !recent_opener_response.hovered() && !popup_hovered {
        //                     ui.memory().close_popup();
        //                 }
        //             },
        //         );

        //         menu::menu(
        //             ui,
        //             crate::LOCALES.lookup(&self.locale_lang.get_lang(), "tools-menu"),
        //             |ui| {
        //                 if ui
        //                     .button(
        //                         crate::LOCALES
        //                             .lookup(&self.locale_lang.get_lang(), "hltas-cleaner"),
        //                     )
        //                     .clicked()
        //                 {
        //                     // TODO show options
        //                     if let Some(current_index) = self.current_tab_index {
        //                         // TODO all error handling here
        //                         let current_hltas = &mut self.tabs[current_index].hltas;
        //                         cleaners::no_dupe_framebulks(current_hltas);
        //                     }
        //                 }
        //             },
        //         );

        //         menu::menu(
        //             ui,
        //             crate::LOCALES.lookup(&self.locale_lang.get_lang(), "options-menu"),
        //             |ui| {
        //                 if ui
        //                     .button(
        //                         crate::LOCALES
        //                             .lookup(&self.locale_lang.get_lang(), "toggle-graphics-editor"),
        //                     )
        //                     .clicked()
        //                 {
        //                     self.graphics_editor = !self.graphics_editor;
        //                 }
        //             },
        //         );
        //     });

        //     ui.separator();

        //     // tabs
        //     let mut stale_tab: Option<usize> = None;
        //     egui::ScrollArea::horizontal().show(ui, |ui| {
        //         ui.horizontal(|ui| {
        //             let mut new_index: Option<usize> = None;
        //             for (index, tab) in self.tabs.iter().enumerate() {
        //                 // tab design
        //                 ui.group(|ui| {
        //                     // if label is clicked, switch to that one
        //                     if ui
        //                         .add(Label::new(&tab.title).sense(Sense::click()))
        //                         .clicked()
        //                     {
        //                         // FIXME not sure why I can't do this
        //                         // self.current_tab_index = Some(index);
        //                         new_index = Some(index);
        //                     }

        //                     let close_button = close_button().small();

        //                     if ui.add(close_button).clicked() {
        //                         // mark as stale
        //                         stale_tab = Some(index);
        //                     }
        //                 });
        //             }

        //             if let Some(_) = new_index {
        //                 self.current_tab_index = new_index;
        //             }
        //         });
        //     });

        //     // remove stale tab
        //     if let Some(stale_tab) = stale_tab {
        //         self.tabs.remove(stale_tab);
        //     }

        //     // fix index if its out of bounds
        //     if let Some(index) = self.current_tab_index {
        //         if self.tabs.len() == 0 {
        //             self.current_tab_index = None;
        //         } else if index >= self.tabs.len() {
        //             self.current_tab_index = Some(self.tabs.len() - 1);
        //         }
        //     }
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // ui.text_edit_multiline(&mut self.raw_content);
        //     // accept file drops
        //     for file in &ui.input().raw.dropped_files {
        //         if let Some(path) = &file.path {
        //             self.open_file(path);
        //         }
        //     }

        //     if let Some(current_tab_index) = self.current_tab_index {
        //         let current_tab = &mut self.tabs[current_tab_index];

        //         if self.graphics_editor {
        //             show_graphics_editor(ui, current_tab);
        //         } else {
        //             show_text_editor(ui, current_tab);
        //         }
        //     }
        // });

        // self.set_current_tab_title();
    }
}
