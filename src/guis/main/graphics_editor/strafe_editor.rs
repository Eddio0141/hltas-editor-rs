use hltas::types::{AutoMovement, StrafeDir, StrafeSettings, StrafeType};
use imgui::{StyleVar, Ui};

use crate::{
    guis::main::tab::StrafeMenuSelection,
    helpers::imgui::{combo_enum::show_combo_enum, list_box_enum::show_list_box_enum},
};

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, HLTASInfo};

pub struct StrafeEditor;

impl FramebulkEditor for StrafeEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let tab_menu_data = framebulk_editor_misc_data.tab_menu_data;

        let initial_x_pos = ui.cursor_pos()[0];

        let strafe_menu_selection = tab_menu_data.strafe_menu_selection_at_mut(index).unwrap();

        if ui.button(format!("Strafe tab##{}", index)) {
            *strafe_menu_selection = Some(StrafeMenuSelection::Strafe);
        }

        ui.same_line();

        let key_tab_pos = ui.cursor_screen_pos();
        if ui.button(format!("Key tab##{}", index)) {
            *strafe_menu_selection = Some(StrafeMenuSelection::Keys);
        }

        ui.set_cursor_pos([initial_x_pos, ui.cursor_pos()[1]]);

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

                    // TODO
                    let width = ui.push_item_width(140.0);

                    let list_box_changed = show_list_box_enum(
                        ui,
                        &mut strafe_type_selection,
                        vec![
                            ("Max accel", Some(StrafeType::MaxAccel)),
                            ("Max angle", Some(StrafeType::MaxAngle)),
                            ("Max deccel", Some(StrafeType::MaxDeccel)),
                            ("Const speed", Some(StrafeType::ConstSpeed)),
                            ("None", None),
                        ],
                        &format!("strafe_selector_list_box{}", index),
                    );

                    if list_box_changed {
                        let prev_yaw = match &framebulk.auto_actions.movement {
                            Some(auto_movement) => match auto_movement {
                                AutoMovement::SetYaw(yaw) => Some(*yaw),
                                AutoMovement::Strafe(strafe_settings) => {
                                    match strafe_settings.dir {
                                        StrafeDir::Yaw(yaw) => Some(yaw),
                                        StrafeDir::Line { yaw } => Some(yaw),
                                        _ => None,
                                    }
                                }
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
                                            _ => StrafeDir::Yaw(prev_yaw.unwrap_or(0.0)),
                                        },
                                    }));
                            }
                            None => {
                                framebulk.auto_actions.movement =
                                    prev_yaw.map(AutoMovement::SetYaw);
                            }
                        }
                    }

                    width.pop(ui);

                    list_box_changed
                }
                StrafeMenuSelection::Keys => {
                    // TODO key layout view
                    let keys = &mut framebulk.movement_keys;
                    let forward_edited = ui.checkbox(
                        format!("Forward##strafe_menu_editor{}", index),
                        &mut keys.forward,
                    );
                    ui.same_line();
                    let y_pos_next = ui.cursor_screen_pos()[1];
                    ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                    let up_edited =
                        ui.checkbox(format!("Up##strafe_menu_editor{}", index), &mut keys.up);
                    let left_edited =
                        ui.checkbox(format!("Left##strafe_menu_editor{}", index), &mut keys.left);
                    ui.same_line();
                    let y_pos_next = ui.cursor_screen_pos()[1];
                    ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                    let down_edited =
                        ui.checkbox(format!("Down##strafe_menu_editor{}", index), &mut keys.down);
                    let right_edited = ui.checkbox(
                        format!("Right##strafe_menu_editor{}", index),
                        &mut keys.right,
                    );
                    let back_edited =
                        ui.checkbox(format!("Back##strafe_menu_editor{}", index), &mut keys.back);

                    forward_edited
                        || up_edited
                        || left_edited
                        || down_edited
                        || right_edited
                        || back_edited
                }
            },
            None => unreachable!("strafe_menu_selection is desynced with hltas line count"),
        }
    }

    fn show_minimal(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let tab_menu_data = framebulk_editor_misc_data.tab_menu_data;

        let strafe_menu_selection = tab_menu_data.strafe_menu_selection_at_mut(index).unwrap();

        if ui.button(format!("Strafe##{}", index)) {
            *strafe_menu_selection = Some(StrafeMenuSelection::Strafe);
        }

        ui.same_line();

        if ui.button(format!("Key##{}", index)) {
            *strafe_menu_selection = Some(StrafeMenuSelection::Keys);
        }

        ui.same_line();

        let strafe_keys_edited = if let Some(strafe_menu_selection) = strafe_menu_selection {
            match strafe_menu_selection {
                StrafeMenuSelection::Strafe => {
                    let values = vec![
                        ("Max accel", Some(StrafeType::MaxAccel)),
                        ("Max angle", Some(StrafeType::MaxAngle)),
                        ("Max deccel", Some(StrafeType::MaxDeccel)),
                        ("Const speed", Some(StrafeType::ConstSpeed)),
                        ("None", None),
                    ];

                    let mut strafe_selection = match framebulk.auto_actions.movement {
                        Some(AutoMovement::Strafe(strafe_settings)) => Some(strafe_settings.type_),
                        _ => None,
                    };

                    // TODO
                    let width = ui.push_item_width(219.0);
                    let strafe_selection_edited = show_combo_enum(
                        ui,
                        &mut strafe_selection,
                        values,
                        &format!("strafe_selection{}", index),
                    );
                    width.pop(ui);

                    if strafe_selection_edited {
                        let prev_yaw = match &framebulk.auto_actions.movement {
                            Some(auto_movement) => match auto_movement {
                                AutoMovement::SetYaw(yaw) => Some(*yaw),
                                AutoMovement::Strafe(strafe_settings) => {
                                    match strafe_settings.dir {
                                        StrafeDir::Yaw(yaw) => Some(yaw),
                                        StrafeDir::Line { yaw } => Some(yaw),
                                        _ => None,
                                    }
                                }
                            },
                            None => None,
                        };

                        match strafe_selection {
                            Some(strafe_type) => {
                                framebulk.auto_actions.movement =
                                    Some(AutoMovement::Strafe(StrafeSettings {
                                        type_: strafe_type,
                                        // TODO make this an option to auto select direction for each strafe type
                                        dir: match strafe_type {
                                            StrafeType::MaxDeccel => StrafeDir::Best,
                                            _ => StrafeDir::Yaw(prev_yaw.unwrap_or(0.0)),
                                        },
                                    }));
                            }
                            None => {
                                framebulk.auto_actions.movement =
                                    prev_yaw.map(AutoMovement::SetYaw);
                            }
                        }
                    }

                    strafe_selection_edited
                }
                StrafeMenuSelection::Keys => {
                    let keys = &mut framebulk.movement_keys;
                    let no_spacing_token = ui.push_style_var(StyleVar::ItemSpacing([5.0, 0.0]));

                    let forward_edited =
                        ui.checkbox(format!("w##strafe_menu_editor{}", index), &mut keys.forward);
                    ui.same_line();
                    let left_edited =
                        ui.checkbox(format!("a##strafe_menu_editor{}", index), &mut keys.left);
                    ui.same_line();
                    let back_edited =
                        ui.checkbox(format!("s##strafe_menu_editor{}", index), &mut keys.back);
                    ui.same_line();
                    let right_edited =
                        ui.checkbox(format!("d##strafe_menu_editor{}", index), &mut keys.right);
                    ui.same_line();
                    let up_edited =
                        ui.checkbox(format!("Up##strafe_menu_editor{}", index), &mut keys.up);
                    ui.same_line();
                    let down_edited =
                        ui.checkbox(format!("Dn##strafe_menu_editor{}", index), &mut keys.down);

                    no_spacing_token.pop();

                    forward_edited
                        || up_edited
                        || left_edited
                        || down_edited
                        || right_edited
                        || back_edited
                }
            }
        } else {
            false
        };

        strafe_keys_edited
    }
}
