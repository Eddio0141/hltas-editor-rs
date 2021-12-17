mod cmd_editor;
mod graphics_editor;
mod option_menu;
mod property_some_none_field;
mod property_string_field;
mod tab;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{collections::VecDeque, fs, path::PathBuf};

use crate::helpers::hltas::hltas_to_str;
use crate::helpers::locale::locale_lang::LocaleLang;

use hltas::types::LeaveGroundActionSpeed;
use hltas_cleaner::cleaners;
use imgui::{
    Condition, MenuItem, StyleVar, TabBar, TabItem, TabItemFlags, Ui, Window, WindowFlags,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};

use self::graphics_editor::show_graphics_editor;
use self::option_menu::{show_option_menu, AppOptions, LgagstOption, OptionMenuStatus};
use self::tab::HLTASFileTab;

pub struct MainGUI {
    tabs: Vec<Rc<RefCell<HLTASFileTab>>>,
    current_tab: Option<Rc<RefCell<HLTASFileTab>>>,
    tab_switch_index: Option<usize>,
    // title: String,
    // TODO option to change size
    recent_paths: VecDeque<PathBuf>,
    graphics_editor: bool,
    locale_lang: LocaleLang,
    options_menu_opened: bool,
    options: AppOptions,
    option_menu_status: OptionMenuStatus,
}

impl MainGUI {
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

        if self.recent_paths.len() > self.options.recent_path_size {
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
        if let Some(tab) = &self.current_tab {
            self.save_tab(warn_user, &mut tab.borrow_mut())?;
        }

        Ok(())
    }

    pub fn save_tab(
        &self,
        warn_user: Option<String>,
        tab: &mut HLTASFileTab,
    ) -> Result<(), std::io::Error> {
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

        if let Some(path) = &tab.path {
            // save_path = Some(path.to_owned());
            fs::write(path, hltas_to_str(&tab.hltas))?;
            tab.got_modified = false;
        } else {
            // no file, save as new file
            if let Ok(Some(path)) = Self::ask_hltas_save_location() {
                fs::write(&path, hltas_to_str(&tab.hltas))?;
                tab.title = HLTASFileTab::title_from_path(&path, &self.locale_lang.get_lang());
            }
        }

        Ok(())
    }

    pub fn close_current_tab(&mut self) {
        let remove_index = if let Some(tab) = &self.current_tab {
            if tab.borrow().got_modified
                && self
                    .save_current_tab(Some(String::from(
                        // TODO translation
                        "Would you like to save the modified file?",
                    )))
                    .is_err()
            {
                return;
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

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }

        {
            let mut tab = self.tabs[index].borrow_mut();

            if tab.got_modified
                && self
                    .save_tab(
                        Some(String::from(
                            // TODO translation
                            "Would you like to save the modified file?",
                        )),
                        &mut tab,
                    )
                    .is_err()
            {
                return;
            }
        }

        self.tabs.remove(index);

        // HACK
        if self.tabs.is_empty() {
            self.current_tab = None;
        }
    }

    // pub fn set_current_tab_title(&mut self)

    // fn default_title() -> &'static str {
    //      TODO translation
    //     "HLTAS Editor"
    // }
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
            // title: Self::default_title().to_string(),
            recent_paths: VecDeque::new(),
            graphics_editor: true,
            locale_lang,
            options_menu_opened: false,
            options: AppOptions {
                jump_lgagst_option: LgagstOption {
                    default_selection: LeaveGroundActionSpeed::Optimal,
                    copy_previous_framebulk: true,
                },
                ducktap_lgagst_option: LgagstOption {
                    default_selection: LeaveGroundActionSpeed::Optimal,
                    copy_previous_framebulk: true,
                },
                recent_path_size: 20,
            },
            option_menu_status: OptionMenuStatus::default(),
        }
    }
}

impl MainGUI {
    pub fn show(&mut self, _run: &mut bool, ui: &mut Ui) {
        let window_border_size_token = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
        let window_min_size_token = ui.push_style_var(StyleVar::WindowMinSize([1.0, 1.0]));

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
                        current_tab.borrow_mut().got_modified = true;
                    }
                }
            });

            ui.menu(self.locale_lang.get_str_from_id("options-menu"), || {
                if MenuItem::new(self.locale_lang.get_str_from_id("toggle-graphics-editor"))
                    .build(ui)
                {
                    self.graphics_editor = !self.graphics_editor;
                }
                if MenuItem::new(self.locale_lang.get_str_from_id("open-options-menu")).build(ui) {
                    self.options_menu_opened = !self.options_menu_opened;
                }
            });
        });

        let tab_window_size = {
            // let style = ui.clone_style();
            [
                ui.io().display_size[0],
                20.0
                // style.window_padding[1] + style.frame_padding[1] + 18.0,
                // style.window_min_size[1],
            ]
        };

        let window_padding_size_token = {
            let style = ui.clone_style();
            ui.push_style_var(StyleVar::WindowPadding([style.window_padding[0], 0.0]))
        };

        Window::new("tab_window")
            .position([0.0, ui.frame_height()], Condition::Always)
            .size(tab_window_size, Condition::Always)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .scrollable(false)
            .build(ui, || {
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
                                self.current_tab = Some(Rc::clone(tab));
                            }

                            if tab.borrow().got_modified {
                                flags = flags.union(TabItemFlags::UNSAVED_DOCUMENT);
                            }

                            flags
                        };

                        let mut opened = true;

                        TabItem::new(format!("{}##tab_{}", &tab.borrow().title, i))
                            .flags(flags)
                            .opened(&mut opened)
                            .build(ui, || {
                                if let Some(current_tab) = &self.current_tab {
                                    if current_tab.as_ptr() != tab.as_ptr() {
                                        new_tab = Some(Rc::clone(tab));
                                    }
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
                        self.close_tab(stale_index);
                    }
                });
            });

        let main_window_size = {
            let display_size = ui.io().display_size;
            [
                display_size[0],
                display_size[1] - (ui.frame_height() + tab_window_size[1]),
            ]
        };

        Window::new("main_window")
            .position(
                [0.0, ui.frame_height() + tab_window_size[1]],
                Condition::Always,
            )
            .size(main_window_size, Condition::Always)
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .horizontal_scrollbar(true)
            .build(ui, || {
                if self.graphics_editor {
                    if let Some(tab) = &self.current_tab {
                        show_graphics_editor(ui, &mut tab.borrow_mut(), &self.options);
                    }
                } else {
                    // show_text_editor(ui);
                }
            });

        window_padding_size_token.pop();

        let mut options_menu_opened = self.options_menu_opened;

        if self.options_menu_opened {
            Window::new("options")
                .flags(if self.option_menu_status.modified() {
                    WindowFlags::UNSAVED_DOCUMENT
                } else {
                    WindowFlags::empty()
                })
                // TODO force focus in a less hacky way
                .focused(true)
                .opened(&mut options_menu_opened)
                .position(
                    {
                        let display_size = ui.io().display_size;
                        [display_size[0] * 0.5, display_size[1] * 0.5]
                    },
                    Condition::Appearing,
                )
                .resizable(false)
                .scrollable(false)
                .scroll_bar(false)
                .position_pivot([0.5, 0.5])
                .size([500.0, 300.0], Condition::Always)
                .build(ui, || {
                    show_option_menu(ui, &mut self.options, &mut self.option_menu_status);
                });
        }

        if !options_menu_opened {
            // undo unsaved stuff
            if let Some(option_menu_before) = self.option_menu_status.option_menu_before() {
                self.options = option_menu_before.clone();
            }
            self.option_menu_status = OptionMenuStatus::default();
        }

        self.options_menu_opened = options_menu_opened;

        #[cfg(debug_assertions)]
        ui.show_demo_window(&mut true);

        window_border_size_token.pop();
        window_min_size_token.pop();
    }
}
