mod frametime_changer;
mod selectable_hltas_button;
mod strafe_key_selector;
mod tab;
mod text_editor;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::helpers::hltas::hltas_to_str;
use crate::helpers::locale::locale_lang::LocaleLang;
use crate::support::System;

use hltas_cleaner::cleaners;
use imgui::{Condition, MenuItem, TabBar, TabItem, TabItemFlags, Ui, Window};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use self::tab::HLTASFileTab;

pub struct MainGUI {
    tabs: Vec<Rc<RefCell<HLTASFileTab>>>,
    current_tab: Option<Rc<RefCell<HLTASFileTab>>>,
    tab_switch_index: Option<usize>,
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
        let new_tab = HLTASFileTab::new_file(&self.locale_lang.get_lang());
        self.tabs.push(Rc::new(RefCell::new(new_tab)));
        // TODO make it an option to auto select new tab?
        self.tab_switch_index = Some(self.tabs.len() - 1);
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
        for (i, tab) in self.tabs.iter().enumerate() {
            if let Some(tab_path) = &tab.borrow().path {
                if tab_path == path {
                    // dupe found
                    self.tab_switch_index = Some(i);
                    return;
                }
            }
        }

        if let Ok(file_content) = fs::read_to_string(&path) {
            match HLTASFileTab::open_path(path, &file_content) {
                Ok(tab) => {
                    self.tabs.push(Rc::new(RefCell::new(tab)));

                    self.tab_switch_index = Some(self.tabs.len() - 1);

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

    pub fn save_current_tab(&self, warn_user: Option<String>) -> Result<(), std::io::Error> {
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

        if let Some(tab) = &self.current_tab {
            if let Some(path) = &tab.borrow().path {
                // save_path = Some(path.to_owned());
                fs::write(path, hltas_to_str(&tab.borrow().hltas))?;
            } else {
                // no file, save as new file
                if let Ok(path) = Self::ask_hltas_save_location() {
                    if let Some(path) = path {
                        fs::write(&path, hltas_to_str(&tab.borrow().hltas))?;
                        tab.borrow_mut().title =
                            HLTASFileTab::title_from_path(&path, &self.locale_lang.get_lang());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn close_current_tab(&mut self) {
        let remove_index = if let Some(tab) = &self.current_tab {
            if tab.borrow().got_modified {
                if let Err(_) = self.save_current_tab(Some(String::from(
                    "Would you like to save the modified file?",
                ))) {
                    return;
                }
            }

            self.tabs.iter().position(|t| t.as_ptr() == tab.as_ptr())
        } else {
            None
        };

        if let Some(remove_index) = remove_index {
            self.tabs.remove(remove_index);
            self.current_tab = None;
        }
    }

    // pub fn close_tab(&mut self, index: usize) {
    //     if index >= self.tabs.len() {
    //         return;
    //     }

    //     let current_tab = &self.tabs[index];

    //     if current_tab.got_modified {
    //         if let Ok(_) = self.save_current_tab(Some(
    //             "Would you like to save the modified file?".to_string(),
    //         )) {
    //             self.tabs.remove(index);
    //         }
    //         // else do nothing since we can't close the tab
    //     } else {
    //         self.tabs.remove(index);
    //     }
    // }

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
        let locale_lang = LocaleLang::new(None);
        let tabs = vec![Rc::new(RefCell::new(HLTASFileTab::new_file(
            &locale_lang.get_lang(),
        )))];
        let current_tab = Some(Rc::clone(&tabs[0]));

        Self {
            tabs,
            current_tab,
            tab_switch_index: None,
            title: Self::default_title().to_string(),
            recent_paths: VecDeque::new(),
            graphics_editor: true,
            locale_lang,
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
        ui.main_menu_bar(|| {
            ui.menu(self.locale_lang.get_str_from_id("file-menu"), || {
                // TODO shortcut keys
                // if MenuItem::new("New").shortcut("Ctrl+O").build(ui) || (ui.io().keys_down[VirtualKeyCode::O as usize] && ui.io().key_ctrl ){
                if MenuItem::new(self.locale_lang.get_str_from_id("new-file")).build(ui) {
                    self.new_file();
                }
                if MenuItem::new(self.locale_lang.get_str_from_id("open-file")).build(ui) {
                    self.open_file_by_dialog();
                }
                if MenuItem::new(self.locale_lang.get_str_from_id("save-file")).build(ui) {
                    // TODO error handle
                    self.save_current_tab(None).ok();
                }
                if MenuItem::new(self.locale_lang.get_str_from_id("close-file")).build(ui) {
                    self.close_current_tab();
                }

                ui.menu(self.locale_lang.get_str_from_id("recent-files"), || {
                    // I need to loop through the whole list to render them anyway
                    let mut opened_file = None;
                    for recent_path in &self.recent_paths {
                        if MenuItem::new(format!("{:?}", recent_path.as_os_str())).build(ui) {
                            // TODO can I make this better
                            opened_file = Some(recent_path.clone());
                        }
                    }

                    if let Some(opened_file) = opened_file {
                        self.open_file(&opened_file);
                    }
                });
            });

            ui.menu(self.locale_lang.get_str_from_id("tools-menu"), || {
                if MenuItem::new(self.locale_lang.get_str_from_id("hltas-cleaner")).build(ui) {
                    // TODO show options
                    if let Some(current_tab) = &self.current_tab {
                        cleaners::no_dupe_framebulks(&mut current_tab.borrow_mut().hltas);
                    }
                }
            });

            ui.menu(self.locale_lang.get_str_from_id("options-menu"), || {
                if MenuItem::new(self.locale_lang.get_str_from_id("toggle-graphics-editor"))
                    .build(ui)
                {
                    self.graphics_editor = !self.graphics_editor;
                }
            });
        });

        let window_size = {
            let mut size = ui.io().display_size;
            size[1] -= ui.frame_height();
            size
        };

        Window::new("main_window")
            .position([0.0, ui.frame_height()], Condition::Always)
            .size(window_size, Condition::Always)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .build(ui, || {
                ui.group(|| {
                    TabBar::new("file_tabs").reorderable(true).build(ui, || {
                        // TODO make this better?
                        let mut new_tab = None;
                        let mut stale_tab = None;

                        for (i, tab) in self.tabs.iter().enumerate() {
                            let flags = {
                                let mut flags = TabItemFlags::empty();

                                let select_this_tab = match self.tab_switch_index {
                                    Some(index) => index == i,
                                    None => false,
                                };

                                if select_this_tab {
                                    flags = flags.union(TabItemFlags::SET_SELECTED);

                                    self.tab_switch_index = None;
                                }

                                flags
                            };

                            let mut opened = true;

                            TabItem::new(format!("{}#{}", &tab.borrow().title, i))
                                .opened(&mut opened)
                                .flags(flags)
                                .build(ui, || {
                                    if let Some(current_tab) = &self.current_tab {
                                        if current_tab.as_ptr() != tab.as_ptr() {
                                            new_tab = Some(Rc::clone(tab));
                                        }
                                    }
                                    // current_tab = Some(Rc::clone(tab));
                                    if self.graphics_editor {
                                        if let Some(path) = &tab.borrow().path {
                                            ui.text(format!("{:?}", path));
                                        }
                                    } else {
                                        // show_text_editor(ui, current_tab);
                                    }
                                });

                            if !opened {
                                stale_tab = Some(i);
                            }
                        }

                        if let Some(current_tab) = new_tab {
                            self.current_tab = Some(current_tab);
                        }

                        if let Some(stale_index) = stale_tab {
                            self.tabs.remove(stale_index);
                        }
                    });
                });

                //     // let mut stale_tab: Option<usize> = None;

                //     // let mut new_index: Option<usize> = None;
                //     // for (index, tab) in self.tabs.iter().enumerate() {
                //     //     // tab design
                //     //     ui.group(|ui| {
                //     //         // if label is clicked, switch to that one
                //     //         if ui
                //     //             .add(Label::new(&tab.title).sense(Sense::click()))
                //     //             .clicked()
                //     //         {
                //     //             // self.current_tab_index = Some(index);
                //     //             new_index = Some(index);
                //     //         }

                //     //         let close_button = close_button().small();

                //     //         if ui.add(close_button).clicked() {
                //     //             // mark as stale
                //     //             stale_tab = Some(index);
                //     //         }
                //     //     });
                //     // }

                //     // if let Some(_) = new_index {
                //     //     self.current_tab_index = new_index;
                //     // }

                //     // // remove stale tab
                //     // if let Some(stale_tab) = stale_tab {
                //     //     self.tabs.remove(stale_tab);
                //     // }

                //     // // fix index if its out of bounds
                //     // if let Some(index) = self.current_tab_index {
                //     //     if self.tabs.len() == 0 {
                //     //         self.current_tab_index = None;
                //     //     } else if index >= self.tabs.len() {
                //     //         self.current_tab_index = Some(self.tabs.len() - 1);
                //     //     }
                //     // }
                // });
            });

        // ui.show_default_style_editor();

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // ui.text_edit_multiline(&mut self.raw_content);
        //     // accept file drops
        //     for file in &ui.input().raw.dropped_files {
        //         if let Some(path) = &file.path {
        //             self.open_file(path);
        //         }
        //     }
        // });

        // self.set_current_tab_title();
    }
}
