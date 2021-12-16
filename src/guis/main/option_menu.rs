use hltas::types::LeaveGroundActionSpeed;
use imgui::Ui;

use crate::guis::list_box_enum::show_list_box_enum;

pub struct AppOptions {
    pub jump_lgagst_option: LgagstOption,
    pub ducktap_lgagst_option: LgagstOption,
}

pub struct LgagstOption {
    pub default_selection: LeaveGroundActionSpeed,
    pub copy_previous_framebulk: bool,
}

impl LgagstOption {
    fn show_ui(&mut self, ui: &Ui, id: &str) {
        show_list_box_enum(
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

        ui.checkbox(
            format!("copy previous framebulk##{}", id),
            &mut self.copy_previous_framebulk,
        );
    }
}

pub struct OptionMenuStatus {
    pub category_selection: CategoryStatus,
}

impl Default for OptionMenuStatus {
    fn default() -> Self {
        Self {
            category_selection: CategoryStatus::LineOption,
        }
    }
}

pub enum CategoryStatus {
    LineOption,
}

pub fn show_option_menu(
    ui: &Ui,
    app_settings: &mut AppOptions,
    option_menu_status: &mut OptionMenuStatus,
) {
    if ui.button("line options") {
        option_menu_status.category_selection = CategoryStatus::LineOption;
    }

    match option_menu_status.category_selection {
        CategoryStatus::LineOption => {
            ui.text("jump lgagst default option");
            ui.indent();
            app_settings.jump_lgagst_option.show_ui(ui, "jump_lgagst");
            ui.unindent();
            ui.text("ducktap lgagst default option");
            ui.indent();
            app_settings
                .ducktap_lgagst_option
                .show_ui(ui, "ducktap_lgagst");
            ui.unindent();
        }
    }
}
