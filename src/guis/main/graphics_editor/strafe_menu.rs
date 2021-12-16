use hltas::types::{AutoMovement, FrameBulk, StrafeDir, StrafeSettings, StrafeType};
use imgui::Ui;

use crate::guis::{list_box_enum::show_list_box_enum, main::tab::StrafeMenuSelection};

pub fn show_strafe_menu(
    ui: &Ui,
    strafe_menu_selection: &mut Option<StrafeMenuSelection>,
    framebulk: &mut FrameBulk,
    id: &str,
) -> bool {
    if ui.button(format!("Strafe tab##{}", id)) {
        *strafe_menu_selection = Some(StrafeMenuSelection::Strafe);
    }

    ui.same_line();

    let key_tab_pos = ui.cursor_screen_pos();
    if ui.button(format!("Key tab##{}", id)) {
        *strafe_menu_selection = Some(StrafeMenuSelection::Keys);
    }

    match strafe_menu_selection {
        Some(menu_selection) => match menu_selection {
            StrafeMenuSelection::Strafe => {
                // using Some with auto_movement to show the strafetype options with an extra "None" option
                let mut strafe_type_selection = match &framebulk.auto_actions.movement {
                    Some(auto_movement) => match auto_movement {
                        AutoMovement::SetYaw(_) => None,
                        AutoMovement::Strafe(strafe_settings) => Some(strafe_settings.type_),
                    },
                    None => None,
                };

                let strafe_list_box_width_token = ui.push_item_width(140.0);

                let list_box_changed = show_list_box_enum(
                    ui,
                    &mut strafe_type_selection,
                    vec![
                        Some(StrafeType::MaxAccel),
                        Some(StrafeType::MaxAngle),
                        Some(StrafeType::MaxDeccel),
                        Some(StrafeType::ConstSpeed),
                        None,
                    ],
                    vec![
                        "Max accel",
                        "Max angle",
                        "Max deccel",
                        "Const speed",
                        "None",
                    ],
                    format!("strafe_selector_list_box{}", id.to_string()),
                );

                if list_box_changed {
                    let prev_yaw = match &framebulk.auto_actions.movement {
                        Some(auto_movement) => match auto_movement {
                            AutoMovement::SetYaw(yaw) => Some(*yaw),
                            AutoMovement::Strafe(strafe_settings) => match strafe_settings.dir {
                                StrafeDir::Yaw(yaw) => Some(yaw),
                                StrafeDir::Line { yaw } => Some(yaw),
                                _ => None,
                            },
                        },
                        None => None,
                    };

                    match strafe_type_selection {
                        Some(strafe_type) => {
                            framebulk.auto_actions.movement =
                                Some(AutoMovement::Strafe(StrafeSettings {
                                    type_: strafe_type,
                                    // TODO make this an option to auto select direction for each strafe type
                                    dir: match strafe_type {
                                        StrafeType::MaxDeccel => StrafeDir::Best,
                                        _ => {
                                            // TODO store "default" yaw value somewhere
                                            StrafeDir::Yaw(prev_yaw.unwrap_or(0.0))
                                        }
                                    },
                                }));
                        }
                        None => {
                            framebulk.auto_actions.movement = prev_yaw.map(AutoMovement::SetYaw);
                        }
                    }
                }

                strafe_list_box_width_token.pop(ui);

                list_box_changed
            }
            StrafeMenuSelection::Keys => {
                // TODO key layout
                // TODO fix this to work with id so no conflicts
                let keys = &mut framebulk.movement_keys;
                let forward_edited = ui.checkbox("Forward", &mut keys.forward);
                ui.same_line();
                let y_pos_next = ui.cursor_screen_pos()[1];
                ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                let up_edited = ui.checkbox("Up", &mut keys.up);
                let left_edited = ui.checkbox("Left", &mut keys.left);
                ui.same_line();
                let y_pos_next = ui.cursor_screen_pos()[1];
                ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                let down_edited = ui.checkbox("Down", &mut keys.down);
                let right_edited = ui.checkbox("Right", &mut keys.right);
                let back_edited = ui.checkbox("Back", &mut keys.back);

                forward_edited
                    || up_edited
                    || left_edited
                    || down_edited
                    || right_edited
                    || back_edited
            }
        },
        None => unreachable!(),
    }
    // TabBar::new(format!("strafe_menu##{}", id)).build(ui, || {
    //     TabItem::new(format!("strafe tab##{}", id)).build(ui, || {

    //     });
    //     TabItem::new(format!("key tab##{}", id)).build(ui, || {
    //     });
    // });
}
