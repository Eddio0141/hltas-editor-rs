use eframe::egui::{ComboBox, Id, Ui};
use hltas::types::{AutoMovement, StrafeType};

use crate::helpers::hltas::strafe_type_to_str;

pub fn strafe_selector(auto_movement: &mut Option<AutoMovement>, ui: &mut Ui, id: Id) {
    match auto_movement {
        Some(auto_movement) => {
            if let AutoMovement::Strafe(strafe_settings) = auto_movement {
                let strafe_types = vec![
                    StrafeType::MaxAccel,
                    StrafeType::MaxAngle,
                    StrafeType::MaxDeccel,
                    StrafeType::ConstSpeed,
                ];

                ComboBox::from_id_source(id)
                    .selected_text(strafe_type_to_str(&strafe_settings.type_))
                    .show_ui(ui, |ui| {
                        for strafe_type_enum in strafe_types {
                            ui.selectable_value(
                                &mut strafe_settings.type_,
                                strafe_type_enum,
                                strafe_type_to_str(&strafe_type_enum),
                            );
                        }
                    });
            }
        }
        None => {}
    }
}
