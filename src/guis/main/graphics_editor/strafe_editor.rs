use hltas::types::{AutoMovement, Line, StrafeDir, StrafeSettings, StrafeType};
use imgui::{Selectable, Ui};

use crate::{
    guis::main::tab::StrafeMenuSelection,
    helpers::imgui::{
        combo_enum::show_combo_enum_undo_redo_framebulk,
        list_box_enum::show_list_box_enum_undo_redo_framebulk,
    },
};

use super::framebulk_editor::{FramebulkEditor, FramebulkEditorMiscData, FramebulkInfo};

pub struct StrafeEditor;

impl FramebulkEditor for StrafeEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, undo_redo_handler) =
            (misc_data.tab_menu_data, misc_data.undo_redo_handler);

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
                    let width_token = ui.push_item_width(140.0);

                    let list_box_changed = show_list_box_enum_undo_redo_framebulk(
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
                        undo_redo_handler,
                        framebulk,
                        index,
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

                    width_token.pop(ui);

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

                    if forward_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.forward =
                            !framebulk_before.movement_keys.forward;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if up_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.up = !framebulk_before.movement_keys.up;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if left_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.left = !framebulk_before.movement_keys.left;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if down_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.down = !framebulk_before.movement_keys.down;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if right_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.right =
                            !framebulk_before.movement_keys.right;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if back_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.back = !framebulk_before.movement_keys.back;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }

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
        hltas_info: FramebulkInfo,
        misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool {
        let framebulk = hltas_info.framebulk;
        let (tab_menu_data, undo_redo_handler) =
            (misc_data.tab_menu_data, misc_data.undo_redo_handler);

        let selectable_radius = 13.;
        let strafe_keys_button_size = [50., 0.];

        let strafe_menu_selection = tab_menu_data.strafe_menu_selection_at_mut(index).unwrap();

        match strafe_menu_selection {
            Some(strafe_menu_selection) => match strafe_menu_selection {
                StrafeMenuSelection::Strafe => {
                    if ui.button_with_size(format!("Strafe##{}", index), strafe_keys_button_size) {
                        *strafe_menu_selection = StrafeMenuSelection::Keys;
                    }
                }
                StrafeMenuSelection::Keys => {
                    if ui.button_with_size(format!("Key##{}", index), strafe_keys_button_size) {
                        *strafe_menu_selection = StrafeMenuSelection::Strafe;
                    }
                }
            },
            None => unreachable!(
                "strafe_menu_selection at index {} has to be a framebulk so it can't be None",
                index
            ),
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

                    let width_token = ui.push_item_width(
                        (selectable_radius + ui.clone_style().item_spacing[0]) * 6.,
                    );
                    // show_list_box_enum_undo_redo_framebulk
                    let strafe_selection_edited = show_combo_enum_undo_redo_framebulk(
                        ui,
                        &mut strafe_selection,
                        values,
                        &format!("strafe_selection{}", index),
                        undo_redo_handler,
                        framebulk,
                        index,
                    );
                    width_token.pop(ui);

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

                    ui.set_cursor_pos({
                        let cursor_pos = ui.cursor_pos();
                        [
                            cursor_pos[0] + ui.clone_style().item_spacing[0],
                            cursor_pos[1],
                        ]
                    });

                    let forward_edited = Selectable::new(format!("w##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.forward);
                    ui.same_line();
                    let left_edited = Selectable::new(format!("a##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.left);
                    ui.same_line();
                    let back_edited = Selectable::new(format!("s##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.back);
                    ui.same_line();
                    let right_edited = Selectable::new(format!("d##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.right);
                    ui.same_line();
                    let up_edited = Selectable::new(format!("Up##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.up);
                    ui.same_line();
                    let down_edited = Selectable::new(format!("Dn##key_editor{}", index))
                        .size([selectable_radius, selectable_radius])
                        .build_with_ref(ui, &mut keys.down);

                    if forward_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.forward =
                            !framebulk_before.movement_keys.forward;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if up_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.up = !framebulk_before.movement_keys.up;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if left_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.left = !framebulk_before.movement_keys.left;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if down_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.down = !framebulk_before.movement_keys.down;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if right_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.right =
                            !framebulk_before.movement_keys.right;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }
                    if back_edited {
                        let mut framebulk_before = framebulk.to_owned();
                        framebulk_before.movement_keys.back = !framebulk_before.movement_keys.back;
                        undo_redo_handler.edit_line(Line::FrameBulk(framebulk_before), index);
                    }

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
