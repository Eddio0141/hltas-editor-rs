use hltas::types::{AutoMovement, FrameBulk, StrafeDir, StrafeSettings, StrafeType};
use imgui::Ui;

use crate::helpers::hltas::strafe_type_to_str;

#[derive(Clone, Copy, Debug)]
enum StrafeKeyMenuSelection {
    Strafe,
    Key,
}

pub fn strafe_key_selector(framebulk: &mut FrameBulk, ui: &mut Ui/*, id: Id*/) {
    // let mut memory = ui.memory();

    // let menu_selection = memory
    //     .id_data_temp
    //     .get_or_insert_with(id, || {
    //         let check_movement_keys = || {
    //             let movement_keys = framebulk.movement_keys;
    //             if movement_keys.forward
    //                 || movement_keys.left
    //                 || movement_keys.right
    //                 || movement_keys.back
    //                 || movement_keys.up
    //                 || movement_keys.down
    //             {
    //                 StrafeKeyMenuSelection::Key
    //             } else {
    //                 StrafeKeyMenuSelection::Strafe
    //             }
    //         };

    //         match framebulk.auto_actions.movement {
    //             Some(auto_movement) => match auto_movement {
    //                 AutoMovement::SetYaw(_) => check_movement_keys(),
    //                 AutoMovement::Strafe(_) => StrafeKeyMenuSelection::Strafe,
    //             },
    //             None => check_movement_keys(),
    //         }
    //     })
    //     .clone();

    // drop(memory);

    // ui.label(format!("{:?}", &menu_selection));

    // ui.horizontal_top(|ui| {
    //     if ui.button("Strafe").clicked() {}
    //     if ui.button("Key").clicked() {}
    // });

    // // let strafe_key_menu_selection = ui
    // //     .memory()
    // //     .id_data_temp
    // //     .get_or_insert_with(id, || StrafeKeyMenuSelection::Strafe).clone();

    // // match strafe_key_menu_selection {
    // //     StrafeKeyMenuSelection::Strafe => {
    // //         ui.label("strafe");
    // //     }
    // //     StrafeKeyMenuSelection::Key => {
    // //         ui.label("key");
    // //     }
    // // }
}
