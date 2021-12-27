use std::{
    env, fs,
    num::{IntErrorKind, ParseIntError},
    path::PathBuf,
};

use fluent_templates::Loader;
use hltas::types::LeaveGroundActionSpeed;
use home::home_dir;
use imgui::{ColorEdit, ComboBox, InputText, Selectable, StyleColor, Ui};
use serde::{Deserialize, Serialize};

use crate::{
    guis::list_box_enum::show_list_box_enum, helpers::locale::locale_lang::LocaleLang,
    locale::LOCALES,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct AppOptions {
    jump_lgagst_option: LgagstOption,
    ducktap_lgagst_option: LgagstOption,
    recent_path_size: usize,
    #[serde(skip_serializing, skip_deserializing)]
    locale_lang: LocaleLang,
    auto_switch_new_tab: bool,
    default_comment: String,
    comment_colour: [f32; 4],
}

impl AppOptions {
    pub fn get_save_dir() -> Result<PathBuf, std::io::Error> {
        let mut save_dir = match home_dir() {
            Some(home_dir) => home_dir,
            None => env::current_dir()?,
        };

        save_dir.push("hltas-editor");

        if !save_dir.exists() {
            fs::create_dir(&save_dir)?;
        }

        Ok(save_dir)
    }

    pub fn option_path() -> Result<PathBuf, std::io::Error> {
        Ok(Self::get_save_dir()?.join("options.json"))
    }

    pub fn save_options(&self) -> Result<(), std::io::Error> {
        let option_data = serde_json::to_string(self).unwrap();
        fs::write(Self::option_path()?, &option_data)?;
        Ok(())
    }

    pub fn load_options() -> Result<Self, Box<dyn std::error::Error>> {
        let option_data = fs::read_to_string(Self::option_path()?)?;
        Ok(serde_json::from_str(&option_data)?)
    }
}

impl AppOptions {
    /// Get a reference to the app options's jump lgagst option.
    pub fn jump_lgagst_option(&self) -> &LgagstOption {
        &self.jump_lgagst_option
    }

    /// Get a reference to the app options's ducktap lgagst option.
    pub fn ducktap_lgagst_option(&self) -> &LgagstOption {
        &self.ducktap_lgagst_option
    }

    /// Get a reference to the app options's recent path size.
    pub fn recent_path_size(&self) -> usize {
        self.recent_path_size
    }

    /// Get a reference to the app options's locale lang.
    pub fn locale_lang(&self) -> &LocaleLang {
        &self.locale_lang
    }

    /// Get a reference to the app options's auto switch new tab.
    pub fn auto_switch_new_tab(&self) -> bool {
        self.auto_switch_new_tab
    }

    /// Get a reference to the app options's default comment.
    pub fn default_comment(&self) -> &str {
        &self.default_comment
    }

    /// Get a reference to the app options's comment colour.
    pub fn comment_colour(&self) -> [f32; 4] {
        self.comment_colour
    }
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            jump_lgagst_option: Default::default(),
            ducktap_lgagst_option: Default::default(),
            recent_path_size: 20,
            locale_lang: LocaleLang::new(None),
            auto_switch_new_tab: true,
            default_comment: "".to_string(),
            comment_colour: [0.0, 1.0, 0.0, 1.0],
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "LeaveGroundActionSpeed")]
enum LeaveGroundActionSpeedDef {
    Any,
    Optimal,
    OptimalWithFullMaxspeed,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LgagstOption {
    #[serde(with = "LeaveGroundActionSpeedDef")]
    pub default_selection: LeaveGroundActionSpeed,
    pub copy_previous_framebulk: bool,
}

impl Default for LgagstOption {
    fn default() -> Self {
        Self {
            default_selection: LeaveGroundActionSpeed::Optimal,
            copy_previous_framebulk: true,
        }
    }
}

impl LgagstOption {
    fn show_ui(&mut self, ui: &Ui, id: &str) -> bool {
        let lgagst_option_changed = show_list_box_enum(
            ui,
            &mut self.default_selection,
            vec![
                ("no lgagst", LeaveGroundActionSpeed::Any),
                ("lgagst", LeaveGroundActionSpeed::Optimal),
                (
                    "lgagst with max spd",
                    LeaveGroundActionSpeed::OptimalWithFullMaxspeed,
                ),
            ],
            &format!("lgagst_option_lgagst_selection{}", id),
        );

        let copy_prev_framebulk_checkbox_clicked = ui.checkbox(
            format!("copy previous framebulk##{}", id),
            &mut self.copy_previous_framebulk,
        );

        lgagst_option_changed || copy_prev_framebulk_checkbox_clicked
    }
}

pub struct OptionMenu {
    category_selection: Category,
    opened: bool,
    option_menu_before: Option<AppOptions>,
    modified: bool,
}

impl Default for OptionMenu {
    fn default() -> Self {
        Self {
            category_selection: Category::MenuOption,
            option_menu_before: None,
            modified: false,
            opened: false,
        }
    }
}

impl OptionMenu {
    pub fn modified(&self) -> bool {
        self.modified
    }

    pub fn revert(&mut self, app_settings: &mut AppOptions) {
        if let Some(option_menu_before) = &self.option_menu_before {
            *app_settings = option_menu_before.clone();
        }
        self.modified = false;
        self.option_menu_before = None;
    }

    pub fn open(&mut self) {
        *self = Self {
            opened: true,
            ..Default::default()
        };
    }

    /// Get a reference to the option menu's opened.
    pub fn is_opened(&self) -> bool {
        self.opened
    }

    pub fn close(&mut self) {
        self.opened = false;
    }

    pub fn show(&mut self, ui: &Ui, app_settings: &mut AppOptions) {
        // back up option before modifying
        if self.option_menu_before.is_none() {
            self.option_menu_before = Some(app_settings.clone());
        }

        let button_label_pairs = vec![
            ("menu options", Category::MenuOption),
            ("line options", Category::LineOption),
            ("language", Category::Language),
        ];

        for (i, (label, button_enum)) in button_label_pairs.iter().enumerate() {
            let menu_tab_inactive_color =
                if *button_enum != self.category_selection {
                    Some(ui.push_style_color(
                        StyleColor::Button,
                        ui.style_color(StyleColor::TabUnfocused),
                    ))
                } else {
                    None
                };

            if ui.button(label) {
                self.category_selection = *button_enum;
            }

            if let Some(menu_tab_inactive_color) = menu_tab_inactive_color {
                menu_tab_inactive_color.pop();
            }

            if i != button_label_pairs.len() - 1 {
                ui.same_line();
            }
        }

        let modified = match self.category_selection {
            Category::Language => {
                let mut use_system_lang = app_settings.locale_lang.is_using_system_lang();
                let changed_using_system_lang =
                    ui.checkbox("use system language", &mut use_system_lang);
                if changed_using_system_lang {
                    if use_system_lang {
                        app_settings.locale_lang.use_system_lang();
                    } else {
                        app_settings
                            .locale_lang
                            .set_lang(&app_settings.locale_lang().get_lang());
                    }
                }

                let option_menu_changed = ComboBox::new("language##option_menu")
                    .preview_value(app_settings.locale_lang.get_lang().to_string())
                    .build(ui, || {
                        let mut combo_box_changed = false;
                        for locale in LOCALES.locales() {
                            let selectable_clicked =
                                Selectable::new(format!("{}##option_menu", locale.to_string()))
                                    .selected(app_settings.locale_lang.get_lang() == *locale)
                                    .build(ui);

                            if selectable_clicked {
                                app_settings.locale_lang.set_lang(locale);
                                combo_box_changed = true;
                            }
                        }

                        combo_box_changed
                    });

                let option_menu_changed = option_menu_changed.unwrap_or(false);

                changed_using_system_lang || option_menu_changed
            }
            Category::MenuOption => {
                let mut recent_path_size = app_settings.recent_path_size.to_string();
                let recent_path_size_edited =
                    InputText::new(ui, "recent path size", &mut recent_path_size)
                        .chars_decimal(true)
                        .chars_noblank(true)
                        .build();

                if recent_path_size_edited {
                    app_settings.recent_path_size =
                        recent_path_size
                            .parse()
                            .unwrap_or_else(|err: ParseIntError| match err.kind() {
                                IntErrorKind::PosOverflow => usize::MAX,
                                IntErrorKind::NegOverflow => usize::MIN,
                                _ => 0,
                            });
                }

                let auto_switch_new_tab_edited = ui.checkbox(
                    "auto switch to new tab",
                    &mut app_settings.auto_switch_new_tab,
                );

                recent_path_size_edited || auto_switch_new_tab_edited
            }
            Category::LineOption => {
                ui.text("jump lgagst default option");
                ui.indent();
                let jump_lgagst_option_changed =
                    app_settings.jump_lgagst_option.show_ui(ui, "jump_lgagst");
                ui.unindent();
                ui.text("ducktap lgagst default option");
                ui.indent();
                let ducktap_lgagst_option_changed = app_settings
                    .ducktap_lgagst_option
                    .show_ui(ui, "ducktap_lgagst");
                ui.unindent();
                ui.dummy([0.0, 15.0]);
                ui.text("default comment");
                ui.indent();
                // let comment_frame_bg =
                // ui.push_style_color(StyleColor::FrameBg, [0.0, 0.0, 0.0, 0.0]);
                let comment_colour =
                    ui.push_style_color(StyleColor::Text, app_settings.comment_colour);
                let default_comment_changed = InputText::new(
                    ui,
                    "##default_comment_option",
                    &mut app_settings.default_comment,
                )
                .build();
                comment_colour.pop();
                ui.unindent();
                ui.text("comment colour");
                ui.indent();
                let comment_color_changed =
                    ColorEdit::new("comment colour", &mut app_settings.comment_colour)
                        .label(false)
                        .build(ui);
                ui.unindent();

                jump_lgagst_option_changed
                    || ducktap_lgagst_option_changed
                    || default_comment_changed
                    || comment_color_changed
            }
        };

        if modified {
            self.modified = true;
        }

        ui.set_cursor_pos([ui.cursor_pos()[0], {
            let style = ui.clone_style();
            ui.window_size()[1]
                - style.item_spacing[1]
                - ui.calc_text_size("")[1]
                - style.frame_padding[1] * 2.0
        }]);

        if ui.button("Save") {
            self.option_menu_before = None;
            self.modified = false;

            if let Err(err) = app_settings.save_options() {
                native_dialog::MessageDialog::new()
                    .set_title(&app_settings.locale_lang().get_string_from_id("error"))
                    .set_type(native_dialog::MessageType::Error)
                    .set_text(&err.to_string())
                    .show_alert()
                    .ok();
            }
        }
        ui.same_line();
        if ui.button("Cancel") {
            self.revert(app_settings);
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Category {
    MenuOption,
    LineOption,
    Language,
}

impl Default for Category {
    fn default() -> Self {
        Category::MenuOption
    }
}
