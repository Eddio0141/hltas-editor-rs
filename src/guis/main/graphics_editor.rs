use std::num::NonZeroU32;

use hltas::types::{
    AutoMovement, Button, Buttons, ChangeTarget, DuckBeforeCollision, DuckBeforeGround,
    DuckWhenJump, JumpBug, LeaveGroundAction, LeaveGroundActionSpeed, LeaveGroundActionType, Line,
    Seeds, StrafeDir, StrafeSettings, StrafeType, Times, VectorialStrafingConstraints,
};
use imgui::{
    CollapsingHeader, ComboBox, Drag, InputFloat, InputText, Selectable, Slider, StyleColor, Ui,
};

use crate::{
    guis::{radio_button_enum::show_radio_button_enum, x_button::show_x_button},
    helpers::hltas::button_to_str,
};

use super::{
    cmd_editor::cmd_editor_ui,
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

                let command_edited = cmd_editor_ui(ui, cmds, "load commands");

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

    let tab_menu_data = &mut tab.tab_menu_data;

    // very hacky
    let mut lines_edited = false;
    // TODO, only render the text required to save a lot of performance
    for (i, line) in &mut tab.hltas.lines.iter_mut().enumerate() {
        let strafe_menu_selection = &mut tab_menu_data.strafe_menu_selections[i];

        ui.text(format!("{}", i));
        ui.same_line();
        let line_count_offset = ui.cursor_screen_pos()[0];

        let line_edited = match line {
            Line::FrameBulk(framebulk) => {
                ui.group(|| {
                    // TODO translation
                    let set_yaw_text = "set yaw";
                    let set_pitch_text = "set pitch";
                    let yaw_text = "yaw";
                    let pitch_text = "pitch";

                    // yaw pitch menu
                    let yaw_pitch_edited = ui.group(|| {
                        let yaw_pitch_changer_offset =
                            ui.window_content_region_width() * 0.025 + line_count_offset;
                        let yaw_pitch_setter_width = ui.window_content_region_width() * 0.2;

                        let yaw_editor = |yaw| {
                            let x_button_clicked =
                                show_x_button(ui, &format!("yaw_set_close{}", i));

                            ui.same_line();

                            ui.set_cursor_screen_pos([
                                yaw_pitch_changer_offset,
                                ui.cursor_screen_pos()[1],
                            ]);

                            let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
                            let yaw_set_changed = Drag::new(format!("{}##yaw_set{}", yaw_text, i))
                                .speed(0.1)
                                .build(ui, yaw);
                            item_width_token.pop(ui);

                            if x_button_clicked {
                                None
                            } else {
                                Some(yaw_set_changed)
                            }
                        };
                        let yaw_button = |disabled, auto_movement: &mut Option<AutoMovement>| {
                            // ui.disabled returns nothing so hacky work around
                            let mut edited = false;
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
                                    edited = true;
                                }
                            });
                            Some(edited)
                        };

                        let edited_yaw = match &mut framebulk.auto_actions.movement {
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

                        let edited_yaw = match edited_yaw {
                            Some(edited_yaw) => edited_yaw,
                            None => {
                                framebulk.auto_actions.movement = None;
                                true
                            }
                        };

                        let edited_pitch = match &mut framebulk.pitch {
                            Some(pitch) => {
                                let pitch_x_clicked =
                                    show_x_button(ui, &format!("pitch_set_close{}", i));

                                ui.same_line();

                                ui.set_cursor_screen_pos([
                                    yaw_pitch_changer_offset,
                                    ui.cursor_screen_pos()[1],
                                ]);

                                let item_width_token = ui.push_item_width(yaw_pitch_setter_width);
                                let pitch_set_changed = Slider::new(
                                    format!("{}##pitch_set{}", pitch_text, i),
                                    -89.0,
                                    89.0,
                                )
                                .build(ui, pitch);
                                item_width_token.pop(ui);

                                if pitch_x_clicked {
                                    None
                                } else {
                                    Some(pitch_set_changed)
                                }
                            }
                            None => {
                                ui.set_cursor_screen_pos([
                                    yaw_pitch_changer_offset,
                                    ui.cursor_screen_pos()[1],
                                ]);

                                let pitch_set_button_clicked = ui.button_with_size(
                                    format!("{}##pitch_set_button{}", set_pitch_text, i),
                                    [yaw_pitch_setter_width, 0.0],
                                );

                                if pitch_set_button_clicked {
                                    framebulk.pitch = Some(0.0);
                                }

                                Some(pitch_set_button_clicked)
                            }
                        };

                        let edited_pitch = match edited_pitch {
                            Some(edited_pitch) => edited_pitch,
                            None => {
                                framebulk.pitch = None;
                                true
                            }
                        };

                        edited_yaw || edited_pitch
                    });

                    ui.same_line();
                    ui.set_cursor_screen_pos([
                        line_count_offset + ui.window_content_region_width() * 0.28,
                        ui.cursor_screen_pos()[1],
                    ]);

                    // strafe menu
                    let strafe_menu_edited = ui.group(|| {
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
                                        true
                                    } else {
                                        false
                                    }
                                }
                                StrafeMenuSelection::Keys => {
                                    // TODO key layout
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
                        // TabBar::new(format!("strafe_menu##{}", i)).build(ui, || {
                        //     TabItem::new(format!("strafe tab##{}", i)).build(ui, || {

                        //     });
                        //     TabItem::new(format!("key tab##{}", i)).build(ui, || {
                        //     });
                        // });
                    });

                    ui.same_line();
                    ui.set_cursor_screen_pos([
                        line_count_offset + ui.window_content_region_width() * 0.44,
                        ui.cursor_screen_pos()[1],
                    ]);

                    // jump menu
                    let jump_menu_edited = ui.group(|| {
                        let jump_ducktap_menu_width = ui.window_content_region_width() * 0.06;
                        let disabled_text_selectable =
                            |selectable: &dyn Fn(&Ui) -> bool, grey_condition: bool| {
                                let color_token = if !grey_condition {
                                    None
                                } else {
                                    Some(ui.push_style_color(
                                        StyleColor::Text,
                                        ui.style_color(StyleColor::TextDisabled),
                                    ))
                                };

                                let selectable_changed = selectable(ui);

                                if let Some(color_token) = color_token {
                                    color_token.pop();
                                }

                                selectable_changed
                            };

                        let (jump_enabled, ducktap_enabled) =
                            match &framebulk.auto_actions.leave_ground_action {
                                Some(leave_ground_action) => match leave_ground_action.type_ {
                                    LeaveGroundActionType::Jump => (true, false),
                                    LeaveGroundActionType::DuckTap { .. } => (false, true),
                                },
                                None => (false, false),
                            };

                        let jumpbug_enabled = match &framebulk.auto_actions.jump_bug {
                            Some(_) => true,
                            None => false,
                        };

                        ui.text("jump / ducktaps");

                        let duck_tap_selected = disabled_text_selectable(
                            &|ui| {
                                Selectable::new(format!("ducktap##jump_menu{}", i))
                                    .selected(ducktap_enabled)
                                    .size([jump_ducktap_menu_width, 0.0])
                                    .build(ui)
                            },
                            !ducktap_enabled,
                        );

                        let jump_selected = disabled_text_selectable(
                            &|ui| {
                                Selectable::new(format!("autojump##jump_menu{}", i))
                                    .selected(jump_enabled)
                                    .size([jump_ducktap_menu_width, 0.0])
                                    .build(ui)
                            },
                            !jump_enabled,
                        );

                        let jumpbug_selected = disabled_text_selectable(
                            &|ui| {
                                Selectable::new(format!("jumpbug##jump_menu{}", i))
                                    .selected(jumpbug_enabled)
                                    .size([jump_ducktap_menu_width, 0.0])
                                    .build(ui)
                            },
                            !jumpbug_enabled,
                        );

                        ui.dummy([0.0, 15.0]);

                        let mut lgagst_changed = false;
                        ui.disabled(!ducktap_enabled && !jump_enabled, || {
                            let width = ui.window_content_region_width() * 0.14;

                            let lgagst_state = match &mut framebulk.auto_actions.leave_ground_action
                            {
                                Some(leave_ground_action) => Some(&mut leave_ground_action.speed),
                                None => None,
                            };
                            let (lgagst_enabled, lgagst_max_spd_enabled) = match &lgagst_state {
                                Some(leave_ground_action_speed) => match &leave_ground_action_speed
                                {
                                    LeaveGroundActionSpeed::Any => (false, false),
                                    LeaveGroundActionSpeed::Optimal => (true, false),
                                    LeaveGroundActionSpeed::OptimalWithFullMaxspeed => {
                                        (false, true)
                                    }
                                },
                                None => (false, false),
                            };

                            let lgagst_selected =
                                Selectable::new(format!("lgagst##jump_menu{}", i))
                                    .selected(lgagst_enabled)
                                    .size([width, 0.0])
                                    .build(ui);
                            let lgagst_max_spd_selected =
                                Selectable::new(format!("lgagst with max spd##jump_menu{}", i))
                                    .selected(lgagst_max_spd_enabled)
                                    .size([width, 0.0])
                                    .build(ui);

                            if jumpbug_selected {
                                if jumpbug_enabled {
                                    framebulk.auto_actions.jump_bug = None;
                                } else {
                                    framebulk.auto_actions.leave_ground_action = None;
                                    framebulk.auto_actions.jump_bug = Some(JumpBug {
                                        times: Times::UnlimitedWithinFrameBulk,
                                    });
                                }
                            } else if let Some(lgagst_state) = lgagst_state {
                                framebulk.auto_actions.jump_bug = None;

                                // toggle lgagst
                                if lgagst_selected {
                                    if lgagst_enabled {
                                        *lgagst_state = LeaveGroundActionSpeed::Any;
                                    } else {
                                        *lgagst_state = LeaveGroundActionSpeed::Optimal;
                                    }
                                }

                                // toggle lgagst max spd
                                if lgagst_max_spd_selected {
                                    if lgagst_max_spd_enabled {
                                        *lgagst_state = LeaveGroundActionSpeed::Any;
                                    } else {
                                        *lgagst_state =
                                            LeaveGroundActionSpeed::OptimalWithFullMaxspeed;
                                    }
                                }
                            }

                            lgagst_changed =
                                lgagst_selected || lgagst_max_spd_selected || jumpbug_selected;
                        });

                        // this toggles the ducktap state
                        if duck_tap_selected {
                            if ducktap_enabled {
                                framebulk.auto_actions.leave_ground_action = None;
                            } else {
                                // TODO 0ms detector or option to have 0ms by default
                                // TODO option for lgagst on by default
                                // TODO ask about "times" field
                                framebulk.auto_actions.leave_ground_action =
                                    Some(LeaveGroundAction {
                                        speed: match framebulk.auto_actions.leave_ground_action {
                                            Some(leave_ground_action) => leave_ground_action.speed,
                                            None => LeaveGroundActionSpeed::Optimal,
                                        },
                                        times: Times::UnlimitedWithinFrameBulk,
                                        type_: LeaveGroundActionType::DuckTap { zero_ms: true },
                                    })
                            }
                        }

                        // this toggles the jump state
                        if jump_selected {
                            if jump_enabled {
                                framebulk.auto_actions.leave_ground_action = None;
                            } else {
                                // TODO option for lgagst on by default
                                // TODO ask about "times" field
                                framebulk.auto_actions.leave_ground_action =
                                    Some(LeaveGroundAction {
                                        speed: match framebulk.auto_actions.leave_ground_action {
                                            Some(leave_ground_action) => leave_ground_action.speed,
                                            None => LeaveGroundActionSpeed::Optimal,
                                        },
                                        times: Times::UnlimitedWithinFrameBulk,
                                        type_: LeaveGroundActionType::Jump,
                                    })
                            }
                        }

                        duck_tap_selected || jump_selected
                    });

                    ui.same_line();
                    ui.set_cursor_screen_pos([
                        line_count_offset + ui.window_content_region_width() * 0.6,
                        ui.cursor_screen_pos()[1],
                    ]);

                    // duck menu
                    let duck_menu_edited = ui.group(|| {
                        let auto_actions = &mut framebulk.auto_actions;

                        let (mut duck_before_collision, mut duck_before_collision_inc_ceiling) =
                            if let Some(dbc) = &auto_actions.duck_before_collision {
                                (true, dbc.including_ceilings)
                            } else {
                                (false, false)
                            };

                        let mut duck_before_ground =
                            if let Some(_) = &auto_actions.duck_before_ground {
                                true
                            } else {
                                false
                            };

                        let mut duck_when_jump = if let Some(_) = &auto_actions.duck_when_jump {
                            true
                        } else {
                            false
                        };

                        ui.text("auto duck");

                        let before_collision_changed = ui.checkbox(
                            format!("before collision##{}", i),
                            &mut duck_before_collision,
                        );

                        ui.indent();

                        // HACK lazy way to set this
                        let mut inc_ceiling_changed = false;
                        ui.disabled(!duck_before_collision, || {
                            inc_ceiling_changed = ui.checkbox(
                                format!("+ ceiling##{}", i),
                                &mut duck_before_collision_inc_ceiling,
                            );
                        });

                        ui.unindent();

                        let before_ground_changed =
                            ui.checkbox(format!("before ground##{}", i), &mut duck_before_ground);

                        let when_jump_changed =
                            ui.checkbox(format!("when jump##{}", i), &mut duck_when_jump);

                        if before_collision_changed {
                            if duck_before_collision {
                                auto_actions.duck_before_collision = Some(DuckBeforeCollision {
                                    times: Times::UnlimitedWithinFrameBulk,
                                    including_ceilings: duck_before_collision_inc_ceiling,
                                });
                            } else {
                                auto_actions.duck_before_collision = None;
                            }
                        }

                        if duck_before_collision && inc_ceiling_changed {
                            if let Some(dbc) = &mut auto_actions.duck_before_collision {
                                dbc.including_ceilings = duck_before_collision_inc_ceiling;
                            }
                        }

                        if before_ground_changed {
                            if duck_before_ground {
                                auto_actions.duck_before_ground = Some(DuckBeforeGround {
                                    times: Times::UnlimitedWithinFrameBulk,
                                });
                            } else {
                                auto_actions.duck_before_ground = None;
                            }
                        }

                        if when_jump_changed {
                            if duck_when_jump {
                                auto_actions.duck_when_jump = Some(DuckWhenJump {
                                    times: Times::UnlimitedWithinFrameBulk,
                                });
                            } else {
                                auto_actions.duck_when_jump = None;
                            }
                        }

                        before_collision_changed || inc_ceiling_changed || before_ground_changed
                    });

                    ui.same_line();
                    ui.set_cursor_screen_pos([
                        line_count_offset + ui.window_content_region_width() * 0.75,
                        ui.cursor_screen_pos()[1],
                    ]);

                    // action keys menu
                    let action_keys_menu_edited = ui.group(|| {
                        let action_keys = &mut framebulk.action_keys;

                        ui.text("action keys");

                        let use_changed = ui.checkbox(format!("use##{}", i), &mut action_keys.use_);
                        let attack1_changed =
                            ui.checkbox(format!("attack 1##{}", i), &mut action_keys.attack_1);
                        let attack2_changed =
                            ui.checkbox(format!("attack 2##{}", i), &mut action_keys.attack_2);
                        let reload_changed =
                            ui.checkbox(format!("reload##{}", i), &mut action_keys.reload);

                        use_changed || attack1_changed || attack2_changed || reload_changed
                    });

                    yaw_pitch_edited
                        || strafe_menu_edited
                        || jump_menu_edited
                        || duck_menu_edited
                        || action_keys_menu_edited
                })
            }
            Line::Save(save) => {
                ui.text("save");
                ui.same_line();
                let save_edit_width = ui.push_item_width(ui.window_content_region_width() * 0.5);
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

                let width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);
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
                                    for (j, button_enum) in button_enums.iter().enumerate() {
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

                        if let Some(button_new) = button_editor_result {
                            if let Some(button_new) = button_new {
                                *button = button_new;
                            }
                        }

                        false
                    };

                    let air_left_edited = button_editor(air_left, "air_left");
                    let air_right_edited = button_editor(air_right, "air_right");
                    let ground_left_edited = button_editor(ground_left, "ground_left");
                    let ground_right_edited = button_editor(ground_right, "ground_right");

                    air_left_edited || air_right_edited || ground_left_edited || ground_right_edited
                } else {
                    false
                };

                buttons_toggle_clicked || buttons_edited
            }
            Line::LGAGSTMinSpeed(lgagst_min_spd) => {
                ui.text("lgagst min speed");
                ui.same_line();

                let width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);
                let edited =
                    InputFloat::new(ui, format!("##lgagstminspd_editor{}", i), lgagst_min_spd)
                        .build();
                width_token.pop(ui);

                edited
            }
            Line::Reset { non_shared_seed } => {
                // TODO use the same nonshared seed editor as the one in properties
                ui.text("reset");
                ui.same_line();

                let width_token = ui.push_item_width(ui.window_content_region_width() * 0.25);
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
                let comment_colour = ui.push_style_color(StyleColor::Text, [0.0, 1.0, 0.0, 1.0]);

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
                            let x_clicked = if show_x_button(ui, &format!("tolerance_zero{}", i)) {
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
        };

        if !lines_edited && line_edited {
            lines_edited = true;
        }
    }

    if properties_edited || lines_edited {
        tab.got_modified = true;
    }
}
