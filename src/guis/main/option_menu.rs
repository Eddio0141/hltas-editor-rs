use std::num::{IntErrorKind, ParseIntError};

use fluent_templates::Loader;
use hltas::types::LeaveGroundActionSpeed;
use imgui::{ComboBox, InputText, Selectable, StyleColor, Ui};
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
}

impl Default for AppOptions {
    fn default() -> Self {
        Self {
            jump_lgagst_option: Default::default(),
            ducktap_lgagst_option: Default::default(),
            recent_path_size: 20,
            locale_lang: LocaleLang::new(None),
            auto_switch_new_tab: true,
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

pub struct OptionMenuStatus {
    pub category_selection: CategoryStatus,
    option_menu_before: Option<AppOptions>,
    modified: bool,
    requires_save: bool,
}

impl Default for OptionMenuStatus {
    fn default() -> Self {
        Self {
            category_selection: CategoryStatus::MenuOption,
            option_menu_before: None,
            modified: false,
            requires_save: false,
        }
    }
}

impl OptionMenuStatus {
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

    /// Get a reference to the option menu status's requires save.
    pub fn requires_save(&self) -> bool {
        self.requires_save
    }

    pub fn saved(&mut self) {
        self.requires_save = false;
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum CategoryStatus {
    MenuOption,
    LineOption,
    Language,
}

pub fn show_option_menu(
    ui: &Ui,
    app_settings: &mut AppOptions,
    option_menu_status: &mut OptionMenuStatus,
) {
    // back up option before modifying
    if option_menu_status.option_menu_before.is_none() {
        option_menu_status.option_menu_before = Some(app_settings.clone());
    }

    let button_label_pairs = vec![
        ("menu options", CategoryStatus::MenuOption),
        ("line options", CategoryStatus::LineOption),
        ("language", CategoryStatus::Language),
    ];

    for (i, (label, button_enum)) in button_label_pairs.iter().enumerate() {
        let menu_tab_inactive_color = if *button_enum != option_menu_status.category_selection {
            Some(ui.push_style_color(StyleColor::Button, ui.style_color(StyleColor::TabUnfocused)))
        } else {
            None
        };

        if ui.button(label) {
            option_menu_status.category_selection = *button_enum;
        }

        if let Some(menu_tab_inactive_color) = menu_tab_inactive_color {
            menu_tab_inactive_color.pop();
        }

        if i != button_label_pairs.len() - 1 {
            ui.same_line();
        }
    }

    let modified = match option_menu_status.category_selection {
        CategoryStatus::Language => {
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
        CategoryStatus::MenuOption => {
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
        CategoryStatus::LineOption => {
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

            jump_lgagst_option_changed || ducktap_lgagst_option_changed
        }
    };

    if modified {
        option_menu_status.modified = true;
    }

    ui.set_cursor_pos([ui.cursor_pos()[0], {
        let style = ui.clone_style();
        ui.window_size()[1]
            - style.item_spacing[1]
            - ui.calc_text_size("")[1]
            - style.frame_padding[1] * 2.0
    }]);

    if ui.button("Save") {
        option_menu_status.option_menu_before = None;
        option_menu_status.modified = false;
        option_menu_status.requires_save = true;
    }
    ui.same_line();
    if ui.button("Cancel") {
        option_menu_status.revert(app_settings);
    }
}
