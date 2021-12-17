use std::num::{IntErrorKind, ParseIntError};

use hltas::types::LeaveGroundActionSpeed;
use imgui::{InputText, Ui};

use crate::guis::list_box_enum::show_list_box_enum;

#[derive(Clone)]
pub struct AppOptions {
    pub jump_lgagst_option: LgagstOption,
    pub ducktap_lgagst_option: LgagstOption,
    pub recent_path_size: usize,
}

#[derive(Clone)]
pub struct LgagstOption {
    pub default_selection: LeaveGroundActionSpeed,
    pub copy_previous_framebulk: bool,
}

impl LgagstOption {
    fn show_ui(&mut self, ui: &Ui, id: &str) -> bool {
        let lgagst_option_changed = show_list_box_enum(
            ui,
            &mut self.default_selection,
            vec![
                LeaveGroundActionSpeed::Any,
                LeaveGroundActionSpeed::Optimal,
                LeaveGroundActionSpeed::OptimalWithFullMaxspeed,
            ],
            vec!["no lgagst", "lgagst", "lgagst with max spd"],
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
}

impl Default for OptionMenuStatus {
    fn default() -> Self {
        Self {
            category_selection: CategoryStatus::MenuOption,
            option_menu_before: None,
            modified: false,
        }
    }
}

impl OptionMenuStatus {
    pub fn modified(&self) -> bool {
        self.modified
    }

    /// Get a reference to the option menu status's option menu before.
    pub fn option_menu_before(&self) -> Option<&AppOptions> {
        self.option_menu_before.as_ref()
    }
}

pub enum CategoryStatus {
    MenuOption,
    LineOption,
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

    if ui.button("menu options") {
        option_menu_status.category_selection = CategoryStatus::MenuOption;
    }
    ui.same_line();
    if ui.button("line options") {
        option_menu_status.category_selection = CategoryStatus::LineOption;
    }

    let modified = match option_menu_status.category_selection {
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

            recent_path_size_edited
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
    }
    ui.same_line();
    if ui.button("Cancel") {
        if let Some(option_menu_before) = option_menu_status.option_menu_before.clone() {
            *app_settings = option_menu_before;
        }
        option_menu_status.option_menu_before = None;
        option_menu_status.modified = false;
    }
}
