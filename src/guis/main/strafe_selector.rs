use eframe::egui::{ComboBox, Id, Ui};
use hltas::types::{AutoMovement, StrafeDir, StrafeSettings, StrafeType};

use crate::helpers::hltas::strafe_type_to_str;

pub fn strafe_selector(auto_movement: &mut Option<AutoMovement>, ui: &mut Ui, id: Id) {
    let mut changable_strafe_settings = {
        match auto_movement {
            Some(auto_movement) => match auto_movement {
                AutoMovement::SetYaw(_) => None,
                AutoMovement::Strafe(strafe_settings) => Some(strafe_settings.type_),
            },
            None => None,
        }
    };

    let strafe_types = vec![
        None,
        Some(StrafeType::MaxAccel),
        Some(StrafeType::MaxAngle),
        Some(StrafeType::MaxDeccel),
        Some(StrafeType::ConstSpeed),
    ];

    let strafe_type_str = match &mut changable_strafe_settings {
        Some(strafe_type) => strafe_type_to_str(strafe_type),
        None => "",
    };

    let changed_selection = ComboBox::from_id_source(id)
        .selected_text(strafe_type_str)
        .show_ui(ui, |ui| {
            let mut changed = false;
            for strafe_type in strafe_types {
                let changed_value = ui
                    .selectable_value(
                        &mut changable_strafe_settings,
                        strafe_type,
                        match &strafe_type {
                            Some(strafe_type) => strafe_type_to_str(strafe_type),
                            None => "",
                        },
                    )
                    .changed();

                if changed_value {
                    changed = true;
                }
            }
            changed
        })
        .inner;

    let changed_selection = match changed_selection {
        Some(changed_selection) => changed_selection,
        None => false,
    };

    if changed_selection {
        match changable_strafe_settings {
            Some(strafe_type) => {
                // get existing yaw if it exists
                // if not, use 0
                let original_yaw = match auto_movement {
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

                let original_yaw = {
                    match original_yaw {
                        Some(original_yaw) => original_yaw,
                        None => 0.0,
                    }
                };

                // actually set the original auto_movement
                *auto_movement = Some(AutoMovement::Strafe(StrafeSettings {
                    type_: strafe_type,
                    dir: StrafeDir::Yaw(original_yaw),
                }));
            }
            None => *auto_movement = None,
        }
    }
}
