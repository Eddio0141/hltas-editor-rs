mod action_keys_menu;
mod command_menu;
mod duck_menu;
mod frames_menu;
mod jump_menu;
mod strafe_menu;
mod yaw_pitch_menu;

use std::num::NonZeroU32;

use hltas::types::{
    Button, Buttons, Change, ChangeTarget, FrameBulk, Line, Seeds, VectorialStrafingConstraints,
};
use imgui::{
    CollapsingHeader, ComboBox, Drag, InputFloat, InputText, MouseButton, Selectable, StyleColor,
    Ui,
};

use crate::{
    guis::{radio_button_enum::show_radio_button_enum, x_button::show_x_button},
    helpers::hltas::button_to_str,
};

use self::{
    action_keys_menu::show_action_keys_menu, command_menu::show_command_menu,
    duck_menu::show_duck_menu, frames_menu::show_frames_menu, jump_menu::show_jump_menu,
    strafe_menu::show_strafe_menu, yaw_pitch_menu::show_yaw_pitch_menu,
};

use super::{
    cmd_editor::show_cmd_editor,
    property_some_none_field::{property_some_none_field_ui, PropertyFieldResult},
    property_string_field::property_string_field_ui,
    tab::{HLTASFileTab, StrafeMenuSelection},
};

// TODO drag speed variables stored somewhere in the function for convinience
// TODO am I suppose to have translation for those? maybe for some, not all
// TODO minimal view to limit each line to be easier to read with shortcut
pub fn show_graphics_editor(ui: &Ui, tab: &mut HLTASFileTab) {
    let properties_edited = if CollapsingHeader::new("Properties")
        .default_open(true)
        .build(ui)
    {
        let demo_edited = property_string_field_ui(
            ui,
            &mut tab.hltas.properties.demo,
            true,
            "Demo name",
            "Set demo recording",
            0.5,
        );

        let save_after_edited = property_string_field_ui(
            ui,
            &mut tab.hltas.properties.save,
            true,
            "Save name",
            "Save after hltas",
            0.5,
        );

        // TODO, make this easier to edit
        let ducktap_0ms_edited = property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.frametime_0ms,
            // TODO make this an option
            "0.0000000001".to_string(),
            "Enable 0ms ducktap",
            |frametime| {
                let x_button_clicked = !show_x_button(ui, "frametime");

                ui.same_line();

                let item_width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);

                let input_text_edited = InputText::new(ui, "0ms frametime", frametime)
                    .chars_noblank(true)
                    .chars_decimal(true)
                    .hint("0ms frametime")
                    .build();

                item_width_token.pop(ui);

                PropertyFieldResult {
                    field_enabled: x_button_clicked,
                    edited: input_text_edited,
                }
            },
        );

        // TODO some easy way of increasing shared / nonshared rng
        //  since if people want different rng results, they can just add 1
        let seed_edited = property_some_none_field_ui(
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

                let shared_rng_edited = Drag::new("shared rng")
                    .speed(0.05)
                    .build(ui, &mut seeds.shared);

                ui.same_line();

                ui.text(format!("(mod 256 = {})", seeds.shared % 256));

                ui.same_line();

                let nonshared_rng_edited = Drag::new("non-shared rng")
                    .speed(0.05)
                    .build(ui, &mut seeds.non_shared);

                item_width_token.pop(ui);

                PropertyFieldResult {
                    field_enabled: x_button_clicked,
                    edited: shared_rng_edited || nonshared_rng_edited,
                }
            },
        );

        // TODO better way for this to be showen? maybe a version check?
        // TODO figure out "default"
        let hlstrafe_version_edited = property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.hlstrafe_version,
            NonZeroU32::new(3).unwrap(),
            "set hlstrafe version",
            |hlstrafe_version| {
                let x_button_clicked = !show_x_button(ui, "hlstrafe_version");

                ui.same_line();

                let item_width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);

                let mut hlstrafe_version_string = hlstrafe_version.to_string();

                let hlstrafe_version_edited =
                    if InputText::new(ui, "hlstrafe version", &mut hlstrafe_version_string)
                        .chars_noblank(true)
                        .chars_decimal(true)
                        .hint("hlstrafe version")
                        .build()
                    {
                        if let Ok(str_to_nonzero) = hlstrafe_version_string.parse::<NonZeroU32>() {
                            *hlstrafe_version = str_to_nonzero;
                        }
                        true
                    } else {
                        false
                    };

                item_width_token.pop(ui);

                PropertyFieldResult {
                    field_enabled: x_button_clicked,
                    edited: hlstrafe_version_edited,
                }
            },
        );

        let load_cmds_edited = property_some_none_field_ui(
            ui,
            &mut tab.hltas.properties.load_command,
            String::new(),
            "set hltas load commands",
            |cmds| {
                let x_button_clicked = !show_x_button(ui, "load_commands");

                ui.same_line();

                let command_edited = show_cmd_editor(ui, cmds, "load commands");

                PropertyFieldResult {
                    field_enabled: x_button_clicked,
                    edited: command_edited,
                }
            },
        );

        demo_edited
            || save_after_edited
            || ducktap_0ms_edited
            || seed_edited
            || hlstrafe_version_edited
            || load_cmds_edited
    } else {
        false
    };

    ui.separator();
    ui.text("Lines");

    let new_line_menu_id = "new_line_menu";
    ui.popup(new_line_menu_id, || {
        let button_names = vec![
            "framebulk",
            "save",
            "shared seed",
            "buttons",
            "lgagst min spd",
            "non-shared seed",
            "comment",
            "vectorial strafing",
            "vectorial strafing constraints",
            "change",
            "target yaw override",
        ];

        let button_type = vec![
            // TODO option for what to choose here
            Line::FrameBulk(FrameBulk {
                auto_actions: hltas::types::AutoActions {
                    movement: None,
                    leave_ground_action: None,
                    jump_bug: None,
                    duck_before_collision: None,
                    duck_before_ground: None,
                    duck_when_jump: None,
                },
                movement_keys: hltas::types::MovementKeys {
                    forward: false,
                    left: false,
                    right: false,
                    back: false,
                    up: false,
                    down: false,
                },
                action_keys: hltas::types::ActionKeys {
                    jump: false,
                    duck: false,
                    use_: false,
                    attack_1: false,
                    attack_2: false,
                    reload: false,
                },
                frame_time: "0.001".to_string(),
                pitch: None,
                frame_count: NonZeroU32::new(1).unwrap(),
                console_command: None,
            }),
            // TODO custom save name
            Line::Save("buffer".to_string()),
            // TODO default seed
            Line::SharedSeed(0),
            Line::Buttons(Buttons::Set {
                air_left: Button::Left,
                air_right: Button::Right,
                ground_left: Button::Left,
                ground_right: Button::Right,
            }),
            // TODO default lgagstminspd
            // TODO maybe grab from previous
            Line::LGAGSTMinSpeed(30.0),
            // TODO default seed
            Line::Reset { non_shared_seed: 0 },
            // TODO default comment
            Line::Comment("".to_string()),
            // TODO maybe check previous vectorial strafing and toggle
            Line::VectorialStrafing(false),
            Line::VectorialStrafingConstraints(VectorialStrafingConstraints::VelocityYawLocking {
                tolerance: 0.0,
            }),
            // TODO think about this one
            Line::Change(Change {
                target: ChangeTarget::Yaw,
                final_value: 0.0,
                over: 0.4,
            }),
            Line::TargetYawOverride(vec![0.0]),
        ];

        ui.text("new line menu");

        let half_way_index = button_names.len() / 2;
        for (i, button_name) in button_names.iter().enumerate() {
            if ui.button(button_name) {
                let _type = &button_type[i];

                let line_type_to_strafe_menu_selection = || match _type {
                    Line::FrameBulk(framebulk) => Some(StrafeMenuSelection::new(framebulk)),
                    _ => None,
                };

                match tab.tab_menu_data.right_click_popup_index {
                    Some(mut index) => {
                        index += 1;

                        tab.hltas.lines.insert(index, _type.to_owned());

                        // insert menu data
                        tab.tab_menu_data
                            .strafe_menu_selections
                            .insert(index, line_type_to_strafe_menu_selection());
                    }
                    None => {
                        tab.hltas.lines.push(_type.to_owned());
                        tab.tab_menu_data
                            .strafe_menu_selections
                            .push(line_type_to_strafe_menu_selection());
                    }
                }

                tab.got_modified = true;

                ui.close_current_popup();
            }

            if i != half_way_index {
                ui.same_line();
            }
        }
    });

    if tab.hltas.lines.is_empty() {
        if ui.is_mouse_clicked(MouseButton::Right) {
            tab.tab_menu_data.right_click_popup_index = None;
            ui.open_popup(new_line_menu_id);
        }

        return;
    }

    let window_y = ui.window_size()[1];

    let mut lines_edited = false;
    let mut stale_line = None;
    let mut new_line_menu_opened = false;

    for (i, line) in &mut tab.hltas.lines.iter_mut().enumerate() {
        let strafe_menu_selection = &mut tab.tab_menu_data.strafe_menu_selections[i];

        let is_rendering_line = {
            let scroll_y = ui.scroll_y();
            let cursor_y = ui.cursor_pos()[1];

            // -120.0 pixels to be less agressive
            // it's also the same height as the framebulk line
            cursor_y > scroll_y - 120.0 && cursor_y < scroll_y + window_y
        };

        if is_rendering_line {
            ui.text(format!("{}", i + 1));
            ui.same_line();
            let line_count_offset = ui.cursor_screen_pos()[0];

            // TODO translation
            let mut line_edited = false;
            ui.group(|| {
                line_edited = match line {
                    Line::FrameBulk(framebulk) => {
                        let (
                            yaw_pitch_menu_offset,
                            strafe_menu_offset,
                            jump_menu_offset,
                            duck_menu_offset,
                            action_keys_menu_offset,
                            frames_menu_offset,
                            command_menu_offset,
                        ) = {
                            let window_width = ui.window_content_region_width();

                            let yaw_pitch_menu_width = window_width * 0.2 + 15.0;
                            let strafe_menu_width = 158.0;
                            let jump_menu_width = 65.0 * 2.0 + 16.0;
                            let duck_menu_width = 150.0;
                            let action_keys_menu_width = 100.0;
                            let frames_menu_width = 157.0;

                            let yaw_pitch_menu_offset = line_count_offset + 18.0;
                            let strafe_menu_offset = yaw_pitch_menu_offset + yaw_pitch_menu_width;
                            let jump_menu_offset = strafe_menu_offset + strafe_menu_width;
                            let duck_menu_offset = jump_menu_offset + jump_menu_width;
                            let action_keys_menu_offset = duck_menu_offset + duck_menu_width;
                            let frames_menu_offset =
                                action_keys_menu_offset + action_keys_menu_width;
                            let command_menu_offset = frames_menu_offset + frames_menu_width;

                            (
                                yaw_pitch_menu_offset,
                                strafe_menu_offset,
                                jump_menu_offset,
                                duck_menu_offset,
                                action_keys_menu_offset,
                                frames_menu_offset,
                                command_menu_offset,
                            )
                        };

                        // yaw pitch menu
                        let yaw_pitch_menu_edited = ui.group(|| {
                            show_yaw_pitch_menu(
                                ui,
                                yaw_pitch_menu_offset,
                                framebulk,
                                &i.to_string(),
                            )
                        });

                        ui.same_line();
                        ui.set_cursor_screen_pos([strafe_menu_offset, ui.cursor_screen_pos()[1]]);

                        // strafe menu
                        let strafe_menu_edited = ui.group(|| {
                            show_strafe_menu(ui, strafe_menu_selection, framebulk, &i.to_string())
                        });

                        ui.same_line();
                        ui.set_cursor_screen_pos([jump_menu_offset, ui.cursor_screen_pos()[1]]);

                        // jump menu
                        let jump_menu_edited =
                            ui.group(|| show_jump_menu(ui, framebulk, &i.to_string()));

                        ui.same_line();
                        ui.set_cursor_screen_pos([duck_menu_offset, ui.cursor_screen_pos()[1]]);

                        // duck menu
                        let duck_menu_edited =
                            ui.group(|| show_duck_menu(ui, framebulk, &i.to_string()));

                        ui.same_line();
                        ui.set_cursor_screen_pos([
                            action_keys_menu_offset,
                            ui.cursor_screen_pos()[1],
                        ]);

                        // action keys menu
                        let action_keys_menu_edited =
                            ui.group(|| show_action_keys_menu(ui, framebulk, &i.to_string()));

                        ui.same_line();
                        ui.set_cursor_screen_pos([frames_menu_offset, ui.cursor_screen_pos()[1]]);

                        // frames menu
                        let frames_menu_edited =
                            ui.group(|| show_frames_menu(ui, framebulk, &i.to_string()));

                        ui.same_line();
                        ui.set_cursor_screen_pos([command_menu_offset, ui.cursor_screen_pos()[1]]);

                        // command menu
                        let command_menu_edited =
                            ui.group(|| show_command_menu(ui, framebulk, &i.to_string()));

                        yaw_pitch_menu_edited
                            || strafe_menu_edited
                            || jump_menu_edited
                            || duck_menu_edited
                            || action_keys_menu_edited
                            || frames_menu_edited
                            || command_menu_edited
                    }
                    Line::Save(save) => {
                        ui.text("save");
                        ui.same_line();
                        let save_edit_width =
                            ui.push_item_width(ui.window_content_region_width() * 0.5);
                        // TODO limit save max char size
                        let save_edit_input_edited =
                            InputText::new(ui, format!("##save_edit_input{}", i), save)
                                .chars_noblank(true)
                                .build();
                        save_edit_width.pop(ui);

                        save_edit_input_edited
                    }
                    Line::SharedSeed(shared_seed) => {
                        // TODO use the same seed editor as the one in properties
                        ui.text("seed");
                        ui.same_line();

                        let width_token =
                            ui.push_item_width(ui.window_content_region_width() * 0.25);
                        let seed_edited = Drag::new(format!("##shared_seed_edit{}", i))
                            .speed(0.05)
                            .build(ui, shared_seed);
                        width_token.pop(ui);

                        seed_edited
                    }
                    Line::Buttons(buttons) => {
                        let set_text = "set";
                        let reset_text = "reset";

                        ui.text("buttons");
                        ui.same_line();
                        ui.text(match buttons {
                            Buttons::Reset => reset_text,
                            Buttons::Set { .. } => set_text,
                        });
                        ui.same_line();

                        let buttons_toggle_clicked = if ui.button(match buttons {
                            Buttons::Reset => set_text,
                            Buttons::Set { .. } => reset_text,
                        }) {
                            match buttons {
                                Buttons::Reset => {
                                    *buttons = Buttons::Set {
                                        air_left: Button::Left,
                                        air_right: Button::Right,
                                        ground_left: Button::Left,
                                        ground_right: Button::Right,
                                    }
                                }
                                Buttons::Set { .. } => *buttons = Buttons::Reset,
                            }
                            true
                        } else {
                            false
                        };

                        let buttons_edited = if let Buttons::Set {
                            air_left,
                            air_right,
                            ground_left,
                            ground_right,
                        } = buttons
                        {
                            let button_editor = |button: &mut Button, id| {
                                let button_editor_result =
                                    ComboBox::new(format!("##button_editor{}{}", i, id))
                                        .preview_value(button_to_str(button))
                                        .build(ui, || {
                                            let button_enums = vec![
                                                Button::Forward,
                                                Button::ForwardLeft,
                                                Button::Left,
                                                Button::BackLeft,
                                                Button::Back,
                                                Button::BackRight,
                                                Button::Right,
                                                Button::ForwardRight,
                                            ];

                                            let mut selected_button = None;
                                            for (j, button_enum) in button_enums.iter().enumerate()
                                            {
                                                if Selectable::new(format!(
                                                    "{}##buttons_editor_selectable{}{}{}",
                                                    button_to_str(button_enum),
                                                    i,
                                                    j,
                                                    id
                                                ))
                                                .build(ui)
                                                {
                                                    selected_button = Some(*button_enum);
                                                }
                                            }

                                            selected_button
                                        });

                                if let Some(Some(button_new)) = button_editor_result {
                                    *button = button_new;
                                }

                                false
                            };

                            let air_left_edited = button_editor(air_left, "air_left");
                            let air_right_edited = button_editor(air_right, "air_right");
                            let ground_left_edited = button_editor(ground_left, "ground_left");
                            let ground_right_edited = button_editor(ground_right, "ground_right");

                            air_left_edited
                                || air_right_edited
                                || ground_left_edited
                                || ground_right_edited
                        } else {
                            false
                        };

                        buttons_toggle_clicked || buttons_edited
                    }
                    Line::LGAGSTMinSpeed(lgagst_min_spd) => {
                        ui.text("lgagst min speed");
                        ui.same_line();

                        let width_token =
                            ui.push_item_width(ui.window_content_region_width() * 0.25);
                        let edited = InputFloat::new(
                            ui,
                            format!("##lgagstminspd_editor{}", i),
                            lgagst_min_spd,
                        )
                        .build();
                        width_token.pop(ui);

                        edited
                    }
                    Line::Reset { non_shared_seed } => {
                        // TODO use the same nonshared seed editor as the one in properties
                        ui.text("reset");
                        ui.same_line();

                        let width_token =
                            ui.push_item_width(ui.window_content_region_width() * 0.25);
                        let seed_edited = Drag::new(format!("##nonshared_seed_edit{}", i))
                            .speed(0.05)
                            .build(ui, non_shared_seed);
                        width_token.pop(ui);

                        seed_edited
                    }
                    Line::Comment(comment) => {
                        let comment_frame_bg =
                            ui.push_style_color(StyleColor::FrameBg, [0.0, 0.0, 0.0, 0.0]);
                        // TODO customizable comment colour
                        let comment_colour =
                            ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);

                        let comment_edited =
                            InputText::new(ui, format!("##comment_editor{}", i), comment).build();

                        comment_colour.pop();
                        comment_frame_bg.pop();

                        comment_edited
                    }
                    Line::VectorialStrafing(vectorial_strafing) => {
                        ui.checkbox(format!("Vectorial strafing##{}", i), vectorial_strafing)
                    }
                    Line::VectorialStrafingConstraints(vectorial_strafing_constraints) => {
                        let yaw_tolerance_width = ui.window_content_region_width() * 0.2;

                        let tolerance_ui = |tolerance: &mut f32, zero_button| {
                            if zero_button && *tolerance == 0.0 {
                                if ui.button(format!("Set tolerance##{}", i)) {
                                    *tolerance = 1.0;
                                    true
                                } else {
                                    false
                                }
                            } else {
                                let width_token = ui.push_item_width(yaw_tolerance_width);
                                let drag_edited = Drag::new(format!("##tolerance_drag{}", i))
                                    .speed(0.01)
                                    .display_format("+- %f")
                                    .range(0.01, f32::MAX)
                                    .build(ui, tolerance);
                                width_token.pop(ui);

                                let x_clicked = if zero_button {
                                    ui.same_line();
                                    let x_clicked =
                                        if show_x_button(ui, &format!("tolerance_zero{}", i)) {
                                            *tolerance = 0.0;
                                            true
                                        } else {
                                            false
                                        };

                                    x_clicked
                                } else {
                                    false
                                };

                                drag_edited || x_clicked
                            }
                        };

                        ui.text("target_yaw");
                        ui.same_line();

                        match vectorial_strafing_constraints {
                            // velocity +- ?
                            VectorialStrafingConstraints::VelocityYaw { tolerance } => {
                                ui.text("velocity");
                                ui.same_line();
                                tolerance_ui(tolerance, false)
                            }
                            // velocity_avg
                            VectorialStrafingConstraints::AvgVelocityYaw { tolerance } => {
                                ui.text("velocity_avg");
                                ui.same_line();
                                tolerance_ui(tolerance, false)
                            }
                            // velocity_lock +- ?
                            VectorialStrafingConstraints::VelocityYawLocking { tolerance } => {
                                ui.text("velocity_lock");
                                ui.same_line();
                                tolerance_ui(tolerance, true)
                            }
                            // ? +- ?
                            VectorialStrafingConstraints::Yaw { yaw, tolerance } => {
                                // TODO use same settings as small yaw editor
                                let width_token = ui.push_item_width(yaw_tolerance_width);
                                let drag_edited = Drag::new(format!("##vectorial_yaw_drag{}", i))
                                    .speed(0.05)
                                    .build(ui, yaw);
                                width_token.pop(ui);

                                ui.same_line();
                                let tolerance_edited = tolerance_ui(tolerance, true);

                                drag_edited || tolerance_edited
                            }
                            // from ? to ?
                            VectorialStrafingConstraints::YawRange { from, to } => {
                                ui.text("from");
                                ui.same_line();
                                // TODO read above
                                let width_token = ui.push_item_width(yaw_tolerance_width);
                                let from_edited = Drag::new(format!("##vectorial_from_drag{}", i))
                                    .speed(0.05)
                                    .build(ui, from);
                                width_token.pop(ui);

                                ui.same_line();
                                ui.text("to");
                                ui.same_line();

                                let width_token = ui.push_item_width(yaw_tolerance_width);
                                let to_edited = Drag::new(format!("##vectorial_to_drag{}", i))
                                    .speed(0.05)
                                    .build(ui, to);
                                width_token.pop(ui);

                                from_edited || to_edited
                            }
                        }
                    }
                    Line::Change(change) => {
                        let drag_size = ui.window_content_region_width() * 0.1;

                        ui.text("Change");
                        ui.same_line();
                        let drag_size_token = ui.push_item_width(drag_size);
                        let target_edited = show_radio_button_enum(
                            ui,
                            &mut change.target,
                            vec![
                                ChangeTarget::Yaw,
                                ChangeTarget::Pitch,
                                ChangeTarget::VectorialStrafingYaw,
                            ],
                            vec!["Yaw", "Pitch", "Target Yaw"],
                            format!("change_radio_buttons{}", i),
                            true,
                        );
                        drag_size_token.pop(ui);
                        ui.same_line();
                        ui.text("to");
                        ui.same_line();
                        let drag_size_token = ui.push_item_width(drag_size);
                        let angle_edited = Drag::new(format!("##change_angle{}", i))
                            .speed(0.1)
                            .build(ui, &mut change.final_value);
                        drag_size_token.pop(ui);
                        ui.same_line();
                        ui.text("over");
                        ui.same_line();
                        let drag_size_token = ui.push_item_width(drag_size);
                        let seconds_edited = Drag::new(format!("s##change_over{}", i))
                            .speed(0.1)
                            // TODO change limiter option
                            .range(0.001, f32::MAX)
                            .build(ui, &mut change.over);
                        drag_size_token.pop(ui);

                        target_edited || angle_edited || seconds_edited
                    }
                    Line::TargetYawOverride(target_yaw_override) => {
                        let override_ui_id = format!("target_yaw_override_popup{}", i);

                        let mut edited_target_yaw = false;
                        ui.popup(&override_ui_id, || {
                            // its unlikely the user will manually edit this, so I use an input text editor
                            for (j, yaw) in target_yaw_override.iter_mut().enumerate() {
                                if InputFloat::new(
                                    ui,
                                    format!("##target_yaw_override_input{}{}", i, j),
                                    yaw,
                                )
                                .build()
                                    && !edited_target_yaw
                                {
                                    edited_target_yaw = true;
                                }
                            }
                        });

                        ui.text("target_yaw override");
                        ui.same_line();
                        if ui.button(format!("...##target_yaw_override_open_popup{}", i)) {
                            ui.open_popup(&override_ui_id);
                        }

                        edited_target_yaw
                    }
                }
            });

            let group_rect_min = ui.item_rect_min();
            let group_rect_max = ui.item_rect_max();

            // display line manip tools on selected line
            if ui.is_mouse_hovering_rect([0.0, group_rect_min[1]], [f32::MAX, group_rect_max[1]]) {
                let cursor_pos = ui.cursor_screen_pos();
                let button_color =
                    ui.push_style_color(StyleColor::Button, [0.172549, 0.30196, 0.458823, 1.0]);

                let button_pos = {
                    let min_rect = group_rect_min;
                    [min_rect[0] - 20.0, min_rect[1]]
                };

                ui.set_cursor_screen_pos(button_pos);
                if show_x_button(ui, &format!("remove_line_button{}", i)) {
                    line_edited = true;
                    stale_line = Some(i);
                }

                button_color.pop();
                ui.set_cursor_screen_pos(cursor_pos);

                // check if right click for new line menu
                if !new_line_menu_opened && ui.is_mouse_clicked(MouseButton::Right) {
                    tab.tab_menu_data.right_click_popup_index = Some(i);
                    new_line_menu_opened = true;
                    ui.open_popup(new_line_menu_id);
                }
            } else {
                // TODO check if right click is in lines area
                if !new_line_menu_opened && ui.is_mouse_clicked(MouseButton::Right) {
                    tab.tab_menu_data.right_click_popup_index = None;
                    new_line_menu_opened = true;
                    ui.open_popup(new_line_menu_id);
                }
            }

            let draw_list = ui.get_window_draw_list();
            draw_list
                .add_rect(group_rect_min, group_rect_max, [0.501, 0.501, 0.501, 0.25])
                .build();

            if !lines_edited && line_edited {
                lines_edited = true;
            }
        } else {
            // TODO find out some better way that doesn't need hardcoded values like this
            ui.dummy([
                0.0,
                match line {
                    Line::FrameBulk(_) => 120.0,
                    Line::Save(_) => 19.0,
                    Line::SharedSeed(_) => 19.0,
                    Line::Buttons(buttons) => match buttons {
                        Buttons::Reset => 19.0,
                        Buttons::Set { .. } => 111.0,
                    },
                    Line::LGAGSTMinSpeed(_) => 19.0,
                    Line::Reset { .. } => 19.0,
                    Line::Comment(_) => 19.0,
                    Line::VectorialStrafing(_) => 19.0,
                    Line::VectorialStrafingConstraints(_) => 19.0,
                    Line::Change(_) => 19.0,
                    Line::TargetYawOverride(_) => 19.0,
                },
            ]);
        }
    }

    if let Some(stale_line) = stale_line {
        tab.hltas.lines.remove(stale_line);
        // TODO a better design for tab_menu_data
        tab.tab_menu_data.strafe_menu_selections.remove(stale_line);
    }

    if properties_edited || lines_edited {
        tab.got_modified = true;
    }
}
