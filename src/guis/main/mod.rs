mod cmd_editor;
mod graphics_editor;
mod key_combination;
mod key_state;
mod option_menu;
mod property_some_none_field;
mod property_string_field;
mod tab;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{collections::VecDeque, fs, path::PathBuf};

use imgui::{
    Condition, MenuItem, StyleVar, TabBar, TabItem, TabItemFlags, Ui, Window, WindowFlags,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use winit::event::VirtualKeyCode;

use crate::helpers::hltas::lines_to_str;

use self::graphics_editor::show_graphics_editor;
use self::key_combination::KeyCombination;
use self::key_state::KeyboardState;
use self::option_menu::{AppOptions, OptionMenu};
use self::tab::HLTASFileTab;

pub struct MainGUI {
    tabs: Vec<Rc<RefCell<HLTASFileTab>>>,
    current_tab: Option<Rc<RefCell<HLTASFileTab>>>,
    tab_switch_index: Option<usize>,
    recent_paths: VecDeque<PathBuf>,
    graphics_editor: bool,
    options: AppOptions,
    option_menu: OptionMenu,
    #[cfg(debug_assertions)]
    debug_menu_opened: bool,
    keyboard_state: KeyboardState,
}

impl MainGUI {
    pub fn new_file(&mut self) {
        let new_tab = HLTASFileTab::new_file(&self.options.locale_lang().get_lang());
        self.tabs.push(Rc::new(RefCell::new(new_tab)));

        if self.options.auto_switch_new_tab() {
            self.tab_switch_index = Some(self.tabs.len() - 1);
        }
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

        if self.recent_paths.len() > self.options.recent_path_size() {
            self.recent_paths.pop_front();
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        // check for dupe tab and switch to it if found
        for (i, tab) in self.tabs.iter().enumerate() {
            if let Some(tab_path) = tab.borrow().path() {
                if tab_path == path {
                    // dupe found
                    if self.options.auto_switch_new_tab() {
                        self.tab_switch_index = Some(i);
                    }

                    return;
                }
            }
        }

        if let Ok(file_content) = fs::read_to_string(&path) {
            match HLTASFileTab::open_path(path, &file_content) {
                Ok(tab) => {
                    self.tabs.push(Rc::new(RefCell::new(tab)));

                    if self.options.auto_switch_new_tab() {
                        self.tab_switch_index = Some(self.tabs.len() - 1);
                    }

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
                .set_title(&self.options.locale_lang().get_string_from_id("warning"))
                .set_type(native_dialog::MessageType::Warning)
                .set_text(&warning_msg)
                .show_confirm();

            if let Ok(warning_user_selection) = warning_user_selection {
                if !warning_user_selection {
                    return Ok(());
                }
            }
        }

        tab.write_hltas_to_file(self.options.locale_lang())
    }

    pub fn close_current_tab(&mut self) {
        let remove_index = if let Some(tab) = &self.current_tab {
            let got_modified = tab.borrow().tab_menu_data.is_modified();
            if got_modified
                && self
                    .save_current_tab(Some(
                        self.options
                            .locale_lang()
                            .get_string_from_id("save-file-question"),
                    ))
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

            if !self.tabs.is_empty() {
                self.tab_switch_index = Some(self.tabs.len() - 1);
            }
        }
    }

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }

        {
            let mut tab = self.tabs[index].borrow_mut();

            if tab.tab_menu_data.is_modified()
                && self
                    .save_tab(
                        Some(
                            self.options
                                .locale_lang()
                                .get_string_from_id("save-file-question"),
                        ),
                        &mut tab,
                    )
                    .is_err()
            {
                return;
            }
        }

        self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.current_tab = None;
        }
    }

    pub fn show(&mut self, _: &mut bool, ui: &mut Ui) {
        self.keyboard_state.update(ui.io());

        let window_border_size_token = ui.push_style_var(StyleVar::WindowBorderSize(0.0));
        let window_min_size_token = ui.push_style_var(StyleVar::WindowMinSize([1.0, 1.0]));

        ui.main_menu_bar(|| {
            // TODO better solution
            let new_file_key = KeyCombination::new(VirtualKeyCode::N).ctrl();
            let open_file_key = KeyCombination::new(VirtualKeyCode::O).ctrl();
            let save_file_key = KeyCombination::new(VirtualKeyCode::S).ctrl();
            let close_file_key = KeyCombination::new(VirtualKeyCode::W).ctrl();

            if new_file_key.just_pressed(&self.keyboard_state) {
                self.new_file();
            }
            if open_file_key.just_pressed(&self.keyboard_state) {
                self.open_file_by_dialog();
            }
            if save_file_key.just_pressed(&self.keyboard_state) {
                self.save_current_tab(None).unwrap_or_else(|err| {
                    MessageDialog::new()
                        .set_title(&self.options.locale_lang().get_string_from_id("error"))
                        .set_type(MessageType::Error)
                        .set_text(&err.to_string())
                        .show_alert()
                        .ok();
                });
            }
            if close_file_key.just_pressed(&self.keyboard_state) {
                self.close_current_tab();
            }

            ui.menu(
                self.options.locale_lang().get_string_from_id("file-menu"),
                || {
                    #[cfg(debug_assertions)]
                    if MenuItem::new("debug menu").build(ui) {
                        self.debug_menu_opened = !self.debug_menu_opened;
                    }
                    if MenuItem::new(self.options.locale_lang().get_string_from_id("new-file"))
                        .shortcut(new_file_key.to_string())
                        .build(ui)
                    {
                        self.new_file();
                    }
                    if MenuItem::new(self.options.locale_lang().get_string_from_id("open-file"))
                        .shortcut(open_file_key.to_string())
                        .build(ui)
                    {
                        self.open_file_by_dialog();
                    }
                    if MenuItem::new(self.options.locale_lang().get_string_from_id("save-file"))
                        .shortcut(save_file_key.to_string())
                        .build(ui)
                    {
                        self.save_current_tab(None).unwrap_or_else(|err| {
                            MessageDialog::new()
                                .set_title(&self.options.locale_lang().get_string_from_id("error"))
                                .set_type(MessageType::Error)
                                .set_text(&err.to_string())
                                .show_alert()
                                .ok();
                        });
                    }
                    if MenuItem::new(self.options.locale_lang().get_string_from_id("close-file"))
                        .shortcut(close_file_key.to_string())
                        .build(ui)
                    {
                        self.close_current_tab();
                    }

                    ui.menu(
                        self.options
                            .locale_lang()
                            .get_string_from_id("recent-files"),
                        || {
                            // I need to loop through the whole list to render them anyway
                            let mut opened_file = None;
                            for recent_path in &self.recent_paths {
                                if MenuItem::new(format!("{:?}", recent_path.as_os_str())).build(ui)
                                {
                                    opened_file = Some(recent_path.clone());
                                }
                            }

                            if let Some(opened_file) = opened_file {
                                self.open_file(&opened_file);
                            }
                        },
                    );
                },
            );
            let copy_key = KeyCombination::new(VirtualKeyCode::C).ctrl();
            let select_all_key = KeyCombination::new(VirtualKeyCode::A).ctrl();

            if copy_key.just_pressed(&self.keyboard_state) {
                if let Some(current_tab) = &self.current_tab {
                    ui.set_clipboard_text(lines_to_str(
                        current_tab
                            .borrow()
                            .get_selected_lines()
                            .iter()
                            .map(|&line| line.to_owned())
                            .collect::<Vec<_>>(),
                    ));
                }
            }
            if select_all_key.just_pressed(&self.keyboard_state) {
                if let Some(current_tab) = &self.current_tab {
                    current_tab.borrow_mut().select_all_lines();
                }
            }

            ui.menu(
                self.options.locale_lang().get_string_from_id("edit-menu"),
                || {
                    if MenuItem::new(self.options.locale_lang().get_string_from_id("copy"))
                        .shortcut(copy_key.to_string())
                        .build(ui)
                    {
                        if let Some(current_tab) = &self.current_tab {
                            ui.set_clipboard_text(lines_to_str(
                                current_tab
                                    .borrow()
                                    .get_selected_lines()
                                    .iter()
                                    .map(|&line| line.to_owned())
                                    .collect::<Vec<_>>(),
                            ));
                        }
                    }

                    if MenuItem::new(self.options.locale_lang().get_string_from_id("select-all"))
                        .shortcut(select_all_key.to_string())
                        .build(ui)
                    {
                        if let Some(current_tab) = &self.current_tab {
                            current_tab.borrow_mut().select_all_lines();
                        }
                    }
                },
            );
            // ui.menu(
            //     self.options.locale_lang().get_string_from_id("tools-menu"),
            //     || {
            //         if MenuItem::new(
            //             self.options
            //                 .locale_lang()
            //                 .get_string_from_id("hltas-cleaner"),
            //         )
            //         .build(ui)
            //         {
            //             // TODO show options
            //             // TODO think of how to make the hltas mutable borrow work
            //             // if let Some(current_tab) = &self.current_tab {
            //             //     cleaners::no_dupe_framebulks(&mut current_tab.borrow_mut().hltas);
            //             //     current_tab.borrow_mut().got_modified();
            //             // }
            //         }
            //     },
            // );

            ui.menu(
                self.options
                    .locale_lang()
                    .get_string_from_id("options-menu"),
                || {
                    if MenuItem::new(
                        self.options
                            .locale_lang()
                            .get_string_from_id("toggle-graphics-editor"),
                    )
                    .build(ui)
                    {
                        self.graphics_editor = !self.graphics_editor;
                    }
                    if MenuItem::new(
                        self.options
                            .locale_lang()
                            .get_string_from_id("open-options-menu"),
                    )
                    .build(ui)
                    {
                        self.option_menu.open();
                    }
                },
            );
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
            .bring_to_front_on_focus(!self.option_menu.is_opened())
            .build(ui, || {
                TabBar::new("file_tabs").reorderable(true).build(ui, || {
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

                            if tab.borrow().tab_menu_data.is_modified() {
                                flags = flags.union(TabItemFlags::UNSAVED_DOCUMENT);
                            }

                            flags
                        };

                        let mut opened = true;

                        TabItem::new(format!("{}##tab_{}", &tab.borrow().title(), i))
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
            .bring_to_front_on_focus(!self.option_menu.is_opened())
            .build(ui, || {
                if self.graphics_editor {
                    if let Some(tab) = &self.current_tab {
                        show_graphics_editor(
                            ui,
                            &mut tab.borrow_mut(),
                            &self.options,
                            &self.keyboard_state,
                        );
                    }
                } else {
                    // show_text_editor(ui);
                }
            });

        window_padding_size_token.pop();
        window_border_size_token.pop();
        window_min_size_token.pop();

        {
            let mut options_menu_opened = self.option_menu.is_opened();
            let options = &mut self.options;
            let option_menu = &mut self.option_menu;

            if options_menu_opened {
                Window::new("options##options_menu")
                    .flags(if option_menu.modified() {
                        WindowFlags::UNSAVED_DOCUMENT
                    } else {
                        WindowFlags::empty()
                    })
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
                    .size([500.0, 400.0], Condition::Always)
                    .build(ui, || {
                        option_menu.show(ui, options);
                    });
            }

            if !options_menu_opened {
                option_menu.revert(options);
                option_menu.close();
            }
        }

        #[cfg(debug_assertions)]
        if self.debug_menu_opened {
            ui.show_demo_window(&mut self.debug_menu_opened);
        }
    }

    pub fn init() -> Self {
        let main_gui = MainGUI::default();
        let options = match AppOptions::load_options() {
            Ok(app_options) => app_options,
            Err(err) => {
                MessageDialog::new()
                    .set_title(&main_gui.options.locale_lang().get_string_from_id("error"))
                    .set_type(MessageType::Error)
                    .set_text(&err.to_string())
                    .show_alert()
                    .ok();

                AppOptions::default()
            }
        };

        Self {
            options,
            ..main_gui
        }
    }
}

impl Default for MainGUI {
    // first time opened will always show a new tab
    fn default() -> Self {
        let options = AppOptions::default();
        let tabs = vec![Rc::new(RefCell::new(HLTASFileTab::new_file(
            &options.locale_lang().get_lang(),
        )))];
        let current_tab = Some(Rc::clone(&tabs[0]));

        Self {
            tabs,
            current_tab,
            tab_switch_index: None,
            recent_paths: VecDeque::new(),
            graphics_editor: true,
            options,
            option_menu: OptionMenu::default(),
            #[cfg(debug_assertions)]
            debug_menu_opened: false,
            keyboard_state: KeyboardState::default(),
        }
    }
}
