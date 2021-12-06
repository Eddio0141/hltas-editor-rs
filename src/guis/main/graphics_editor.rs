use std::num::NonZeroU32;

use hltas::types::{AutoMovement, Line, Seeds, StrafeDir, StrafeSettings, StrafeType};
use imgui::{CollapsingHeader, Drag, ImColor32, InputText, Slider, StyleColor, StyleVar, Ui};

use crate::guis::{radio_button_enum::show_radio_button_enum, x_button::show_x_button};

use super::{
    cmd_editor::cmd_editor_ui,
    property_some_none_field::property_some_none_field_ui,
    property_string_field::property_string_field_ui,
    tab::{HLTASFileTab, StrafeMenuSelection},
};

// TODO am I suppose to have translation for those? maybe for some, not all
pub fn show_graphics_editor(ui: &Ui, tab: &mut HLTASFileTab) {
    if CollapsingHeader::new("Properties")
        .default_open(true)
        .build(ui)
    {
        property_string_field_ui(
            ui,
            &mut tab.hltas.properties.demo,
            true,
            "Demo name",
            "Set demo recording",
            0.5,
        );
        property_string_field_ui(
            ui,
            &mut tab.hltas.properties.save,
            true,
            "Save name",
            "Save after hltas",
            0.5,
        );

        // TODO, make this easier to edit
        property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.frametime_0ms,
            // TODO make this an option
            "0.0000000001".to_string(),
            "Enable 0ms ducktap",
            |frametime| {
                let x_button_clicked = !show_x_button(ui, "frametime");

                ui.same_line();

                let item_width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);

                InputText::new(ui, "0ms frametime", frametime)
                    .chars_noblank(true)
                    .chars_decimal(true)
                    .hint("0ms frametime")
                    .build();

                item_width_token.pop(ui);

                x_button_clicked
            },
        );

        // TODO some easy way of increasing shared / nonshared rng
        //  since if people want different rng results, they can just add 1
        property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.seeds,
            Seeds {
                shared: 0,
                non_shared: 0,
            },
            "enable shared / non-shared rng set",
            |seeds| {
                let x_button_clicked = !show_x_button(ui, "seeds");
                ui.same_line();

                let item_width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);

                Drag::new("shared rng")
                    .speed(0.05)
                    .build(ui, &mut seeds.shared);

                ui.same_line();

                ui.text(format!("(mod 256 = {})", seeds.shared % 256));

                ui.same_line();

                Drag::new("non-shared rng")
                    .speed(0.05)
                    .build(ui, &mut seeds.non_shared);

                item_width_token.pop(ui);

                x_button_clicked
            },
        );

        // TODO better way for this to be showen? maybe a version check?
        // TODO figure out "default"
        property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.hlstrafe_version,
            NonZeroU32::new(3).unwrap(),
            "set hlstrafe version",
            |hlstrafe_version| {
                let x_button_clicked = !show_x_button(ui, "hlstrafe_version");

                ui.same_line();

                let item_width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);

                let mut hlstrafe_version_string = hlstrafe_version.to_string();

                if InputText::new(ui, "hlstrafe version", &mut hlstrafe_version_string)
                    .chars_noblank(true)
                    .chars_decimal(true)
                    .hint("hlstrafe version")
                    .build()
                {
                    if let Ok(str_to_nonzero) = hlstrafe_version_string.parse::<NonZeroU32>() {
                        *hlstrafe_version = str_to_nonzero;
                    }
                }

                item_width_token.pop(ui);

                x_button_clicked
            },
        );

        property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.load_command,
            String::new(),
            "set hltas load commands",
            |cmds| {
                let x_button_clicked = !show_x_button(ui, "load_commands");

                ui.same_line();

                cmd_editor_ui(ui, cmds, "load commands");

                x_button_clicked
            },
        );
    }

    ui.separator();
    ui.text("Lines");

    let tab_menu_data = &mut tab.tab_menu_data;

    // TODO, only render the lines required to save a lot of performance

    for (i, line) in &mut tab.hltas.lines.iter_mut().enumerate() {
        let strafe_menu_selection = &mut tab_menu_data.strafe_menu_selections[i];

        ui.text(format!("{}", i));
        ui.same_line();
        let line_count_offset = ui.cursor_screen_pos()[0];

        match line {
            Line::FrameBulk(framebulk) => {
                ui.group(|| {
                    // TODO translation
                    let set_yaw_text = "set yaw";
                    let set_pitch_text = "set pitch";
                    let yaw_text = "yaw";
                    let pitch_text = "pitch";

                    // yaw pitch menu
                    ui.group(|| {
                        let yaw_pitch_changer_offset =
                            ui.window_content_region_width() * 0.025 + line_count_offset;
                        let yaw_pitch_setter_width = ui.window_content_region_width() * 0.2;

                        let yaw_editor = |yaw| {
                            show_x_button(ui, &format!("yaw_set_close{}", i));

                            ui.same_line();

                            ui.set_cursor_screen_pos([
                                yaw_pitch_changer_offset,
                                ui.cursor_screen_pos()[1],
                            ]);

                            let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
                            Drag::new(format!("{}##yaw_set{}", yaw_text, i))
                                .speed(0.1)
                                .build(ui, yaw);
                            item_width_token.pop(ui);
                        };
                        let yaw_button = |disabled, auto_movement: &mut Option<AutoMovement>| {
                            ui.disabled(disabled, || {
                                ui.set_cursor_screen_pos([
                                    yaw_pitch_changer_offset,
                                    ui.cursor_screen_pos()[1],
                                ]);

                                if ui.button_with_size(
                                    format!("{}##yaw_set_button{}", set_yaw_text, i),
                                    [yaw_pitch_setter_width, 0.0],
                                ) {
                                    *auto_movement = Some(AutoMovement::SetYaw(0.0));
                                }
                            });
                        };
                        match &mut framebulk.auto_actions.movement {
                            Some(auto_movement) => match auto_movement {
                                AutoMovement::SetYaw(yaw) => yaw_editor(yaw),
                                AutoMovement::Strafe(strafe_settings) => {
                                    match &mut strafe_settings.dir {
                                        StrafeDir::Yaw(yaw) => yaw_editor(yaw),
                                        StrafeDir::Line { yaw } => yaw_editor(yaw),
                                        _ => yaw_button(true, &mut framebulk.auto_actions.movement),
                                    }
                                }
                            },
                            None => {
                                // show yaw button
                                yaw_button(false, &mut framebulk.auto_actions.movement)
                            }
                        };

                        match &mut framebulk.pitch {
                            Some(pitch) => {
                                show_x_button(ui, &format!("pitch_set_close{}", i));

                                ui.same_line();

                                ui.set_cursor_screen_pos([
                                    yaw_pitch_changer_offset,
                                    ui.cursor_screen_pos()[1],
                                ]);

                                let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
                                Slider::new(format!("{}##pitch_set{}", pitch_text, i), -89.0, 89.0)
                                    .build(ui, pitch);
                                item_width_token.pop(ui);
                            }
                            None => {
                                ui.set_cursor_screen_pos([
                                    yaw_pitch_changer_offset,
                                    ui.cursor_screen_pos()[1],
                                ]);

                                if ui.button_with_size(
                                    format!("{}##pitch_set_button{}", set_pitch_text, i),
                                    [yaw_pitch_setter_width, 0.0],
                                ) {
                                    framebulk.pitch = Some(0.0);
                                }
                            }
                        }
                    });

                    ui.same_line();

                    ui.set_cursor_screen_pos([
                        line_count_offset + ui.window_content_region_width() * 0.28,
                        ui.cursor_screen_pos()[1],
                    ]);

                    // strafe menu
                    ui.group(|| {
                        if ui.button(format!("Strafe tab##{}", i)) {
                            *strafe_menu_selection = Some(StrafeMenuSelection::Strafe);
                        }

                        ui.same_line();

                        let key_tab_pos = ui.cursor_screen_pos();
                        if ui.button(format!("Key tab##{}", i)) {
                            *strafe_menu_selection = Some(StrafeMenuSelection::Keys);
                        }

                        match strafe_menu_selection {
                            Some(menu_selection) => match menu_selection {
                                StrafeMenuSelection::Strafe => {
                                    // using Some with auto_movement to show the strafetype options with an extra "None" option
                                    let mut strafe_type_selection =
                                        match &framebulk.auto_actions.movement {
                                            Some(auto_movement) => match auto_movement {
                                                AutoMovement::SetYaw(_) => None,
                                                AutoMovement::Strafe(strafe_settings) => {
                                                    Some(strafe_settings.type_)
                                                }
                                            },
                                            None => None,
                                        };

                                    if show_radio_button_enum(
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
                                        i.to_string(),
                                        false,
                                    ) {
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
                                                            StrafeType::MaxDeccel => {
                                                                StrafeDir::Best
                                                            }
                                                            _ => {
                                                                StrafeDir::Yaw(match prev_yaw {
                                                                    Some(yaw) => yaw,
                                                                    // TODO store "default" yaw value somewhere
                                                                    None => 0.0,
                                                                })
                                                            }
                                                        },
                                                    }));
                                            }
                                            None => {
                                                framebulk.auto_actions.movement = match prev_yaw {
                                                    Some(yaw) => Some(AutoMovement::SetYaw(yaw)),
                                                    None => None,
                                                };
                                            }
                                        }
                                    }
                                }
                                StrafeMenuSelection::Keys => {
                                    // TODO key layout
                                    let keys = &mut framebulk.movement_keys;
                                    ui.checkbox("Forward", &mut keys.forward);
                                    ui.same_line();
                                    let y_pos_next = ui.cursor_screen_pos()[1];
                                    ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                                    ui.checkbox("Up", &mut keys.up);
                                    ui.checkbox("Left", &mut keys.left);
                                    ui.same_line();
                                    let y_pos_next = ui.cursor_screen_pos()[1];
                                    ui.set_cursor_screen_pos([key_tab_pos[0], y_pos_next]);
                                    ui.checkbox("Down", &mut keys.down);
                                    ui.checkbox("Right", &mut keys.right);
                                    ui.checkbox("Back", &mut keys.back);
                                }
                            },
                            None => unreachable!(),
                        }
                        // TabBar::new(format!("strafe_menu##{}", i)).build(ui, || {
                        //     TabItem::new(format!("strafe tab##{}", i)).build(ui, || {

                        //     });
                        //     TabItem::new(format!("key tab##{}", i)).build(ui, || {
                        //     });
                        // });
                    });
                });
            }
            Line::Save(save) => {}
            Line::SharedSeed(shared_seed) => {}
            Line::Buttons(buttons) => {}
            Line::LGAGSTMinSpeed(lgagst_min_spd) => {}
            Line::Reset { non_shared_seed } => {}
            Line::Comment(comment) => {
                let comment_frame_bg =
                    ui.push_style_color(StyleColor::FrameBg, [0.0, 0.0, 0.0, 0.0]);
                // TODO customizable comment colour
                let comment_colour =
                    ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);

                InputText::new(ui, format!("##comment_editor{}", i), comment).build();

                comment_colour.pop();
                comment_frame_bg.pop();
            }
            Line::VectorialStrafing(vectorial_strafing) => {}
            Line::VectorialStrafingConstraints(vectorial_strafing_constraints) => {}
            Line::Change(change) => {}
            Line::TargetYawOverride(target_yaw_override) => {}
        }
    }
}
