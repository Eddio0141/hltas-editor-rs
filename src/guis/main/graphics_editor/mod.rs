mod action_keys_menu;
mod command_menu;
mod duck_menu;
mod frames_menu;
mod jump_menu;
mod seed_editor;
mod strafe_menu;
mod yaw_pitch_menu;

use std::num::NonZeroU32;

use hltas::types::{
    Button, Buttons, Change, ChangeTarget, Line, Seeds, VectorialStrafingConstraints,
};
use imgui::{
    CollapsingHeader, ComboBox, Drag, InputFloat, InputText, MouseButton, Selectable, StyleColor,
    Ui,
};
use native_dialog::{MessageDialog, MessageType};
use winit::event::VirtualKeyCode;

use crate::{
    guis::{radio_button_enum::show_radio_button_enum, x_button::show_x_button},
    helpers::hltas::{button_to_str, empty_framebulk},
};

use self::{
    action_keys_menu::show_action_keys_menu,
    command_menu::show_command_menu,
    duck_menu::show_duck_menu,
    frames_menu::show_frames_menu,
    jump_menu::show_jump_menu,
    seed_editor::{show_non_shared_seed_editor, show_shared_seed_editor},
    strafe_menu::show_strafe_menu,
    yaw_pitch_menu::show_yaw_pitch_menu,
};

use super::{
    cmd_editor::show_cmd_editor,
    key_state::KeyboardState,
    option_menu::AppOptions,
    property_some_none_field::{property_some_none_field_ui, PropertyFieldResult},
    property_string_field::property_string_field_ui,
    tab::HLTASFileTab,
    zero_ms_editor::show_zero_ms_editor,
};

// TODO drag speed variables stored somewhere in the function for convinience
// TODO am I suppose to have translation for those? maybe for some, not all
// TODO minimal view to limit each line to be easier to read with shortcut
pub fn show_graphics_editor(
    ui: &Ui,
    tab: &mut HLTASFileTab,
    options: &AppOptions,
    keyboard_state: &KeyboardState,
) {
    let draw_list = ui.get_window_draw_list();

    let properties_edited =
        if CollapsingHeader::new(options.locale_lang().get_string_from_id("properties"))
            .default_open(true)
            .build(ui)
        {
            let demo_edited = property_string_field_ui(
                ui,
                &mut tab.hltas_properties_mut().demo,
                true,
                &options.locale_lang().get_string_from_id("demo-name"),
                &options
                    .locale_lang()
                    .get_string_from_id("set-demo-recording"),
                0.5,
            );

            let save_after_edited = property_string_field_ui(
                ui,
                &mut tab.hltas_properties_mut().save,
                true,
                &options.locale_lang().get_string_from_id("save-name"),
                &options.locale_lang().get_string_from_id("save-after-hltas"),
                0.5,
            );

            // TODO, make this easier to edit
            let ducktap_0ms_edited = property_some_none_field_ui(
                ui,
                &mut tab.hltas_properties_mut().frametime_0ms,
                options.default_0ms_frametime().to_string(),
                &options
                    .locale_lang()
                    .get_string_from_id("enable-0ms-ducktap"),
                |frametime| {
                    let x_button_clicked = !show_x_button(ui, "frametime");
                    ui.same_line();

                    ui.text(
                        options
                            .locale_lang()
                            .get_string_from_id("zero-ms-frametime"),
                    );
                    ui.same_line();

                    let item_width_token =
                        ui.push_item_width(ui.window_content_region_width() * 0.25);

                    // will show an error and set to default value
                    let (mut frametime_f32, frametime_f32_edited) = match frametime.parse::<f32>() {
                        Ok(frametime) => (frametime, false),
                        Err(err) => {
                            MessageDialog::new()
                                .set_title(&options.locale_lang().get_string_from_id("error"))
                                .set_text(&format!(
                                    "{}\n{}\n{}",
                                    options
                                        .locale_lang()
                                        .get_string_from_id("frametime-f32-parse-err"),
                                    options
                                        .locale_lang()
                                        .get_string_from_id("setting-default-frametime"),
                                    err
                                ))
                                .set_type(MessageType::Error)
                                .show_alert()
                                .ok();
                            (options.default_0ms_frametime(), true)
                        }
                    };
                    let input_text_edited =
                        show_zero_ms_editor(ui, "field_0ms_editor", &mut frametime_f32);
                    item_width_token.pop(ui);

                    if input_text_edited || frametime_f32_edited {
                        *frametime = frametime_f32.to_string();
                    }

                    PropertyFieldResult {
                        field_enabled: x_button_clicked,
                        edited: input_text_edited || frametime_f32_edited,
                    }
                },
            );
            let seed_edited = property_some_none_field_ui(
                ui,
                &mut tab.hltas_properties_mut().seeds,
                Seeds {
                    shared: 0,
                    non_shared: 0,
                },
                &options
                    .locale_lang()
                    .get_string_from_id("enable-shared-nonshared"),
                |seeds| {
                    let x_button_clicked = !show_x_button(ui, "seeds");
                    ui.same_line();

                    let item_width = ui.window_content_region_width() * 0.25;

                    let shared_rng_edited = show_shared_seed_editor(
                        ui,
                        item_width,
                        "properties",
                        &mut seeds.shared,
                        options.locale_lang(),
                    );
                    ui.same_line();
                    let nonshared_rng_edited = show_non_shared_seed_editor(
                        ui,
                        item_width,
                        "properties",
                        &mut seeds.non_shared,
                        options.locale_lang(),
                    );

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
                &mut tab.hltas_properties_mut().hlstrafe_version,
                NonZeroU32::new(3).unwrap(),
                &options
                    .locale_lang()
                    .get_string_from_id("set-hlstrafe-version"),
                |hlstrafe_version| {
                    let x_button_clicked = !show_x_button(ui, "hlstrafe_version");

                    ui.same_line();

                    let item_width_token =
                        ui.push_item_width(ui.window_content_region_width() * 0.25);

                    let mut hlstrafe_version_string = hlstrafe_version.to_string();

                    let hlstrafe_version_edited = if InputText::new(
                        ui,
                        options.locale_lang().get_string_from_id("hlstrafe-version"),
                        &mut hlstrafe_version_string,
                    )
                    .chars_noblank(true)
                    .chars_decimal(true)
                    .hint(options.locale_lang().get_string_from_id("hlstrafe-version"))
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
                &mut tab.hltas_properties_mut().load_command,
                String::new(),
                &options
                    .locale_lang()
                    .get_string_from_id("set-hltas-load-commands"),
                |cmds| {
                    let x_button_clicked = !show_x_button(ui, "load_commands");

                    ui.same_line();

                    let command_edited = show_cmd_editor(
                        ui,
                        cmds,
                        &options.locale_lang().get_string_from_id("load-commands"),
                        options.locale_lang(),
                    );

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
    ui.text(options.locale_lang().get_string_from_id("lines"));

    let new_line_menu_id = "new_line_menu";
    ui.popup(new_line_menu_id, || {
        let previous_lines = match tab.tab_menu_data.right_click_popup_index() {
            Some(index) => Some(&tab.hltas.lines[..index]),
            None => {
                if tab.hltas.lines.is_empty() {
                    None
                } else {
                    Some(&tab.hltas.lines[..])
                }
            }
        };

        let new_framebulk_with_frametime_framecount = || {
            let default_frametime = &options.default_frametime().to_string();

            let frametime = if let Some(previous_lines) = previous_lines {
                if let Some(Line::FrameBulk(framebulk)) = &previous_lines
                    .iter()
                    .rev()
                    .find(|line| matches!(line, Line::FrameBulk(..)))
                {
                    &framebulk.frame_time
                } else {
                    default_frametime
                }
            } else {
                default_frametime
            };

            let frame_count = NonZeroU32::new(1).unwrap();

            Line::FrameBulk(empty_framebulk(frametime, frame_count))
        };

        // TODO option for what to choose here
        let name_and_types = vec![
            ("framebulk", {
                let new_framebulk = new_framebulk_with_frametime_framecount();

                if options.copy_previous_framebulk() {
                    if let Some(previous_lines) = previous_lines {
                        if let Some(previous_framebulk) = previous_lines
                            .iter()
                            .rev()
                            .find(|line| matches!(line, Line::FrameBulk(..)))
                        {
                            previous_framebulk.to_owned()
                        } else {
                            new_framebulk
                        }
                    } else {
                        new_framebulk
                    }
                } else {
                    new_framebulk
                }
            }),
            ("empty framebulk", new_framebulk_with_frametime_framecount()),
            ("save", Line::Save(options.save_buffer_name().to_string())),
            ("shared seed", Line::SharedSeed(0)),
            (
                "buttons",
                Line::Buttons(Buttons::Set {
                    air_left: Button::Left,
                    air_right: Button::Right,
                    ground_left: Button::Left,
                    ground_right: Button::Right,
                }),
            ),
            ("lgagst min spd", {
                let options_lgagst_min_spd = Line::LGAGSTMinSpeed(options.lgagst_min_speed());

                if options.lgagst_min_speed_grab_prev() {
                    if let Some(previous_lines) = previous_lines {
                        if let Some(Line::LGAGSTMinSpeed(lgagst_min_spd)) = previous_lines
                            .iter()
                            .rev()
                            .find(|line| matches!(line, Line::LGAGSTMinSpeed(_)))
                        {
                            Line::LGAGSTMinSpeed(*lgagst_min_spd)
                        } else {
                            options_lgagst_min_spd
                        }
                    } else {
                        options_lgagst_min_spd
                    }
                } else {
                    options_lgagst_min_spd
                }
            }),
            ("non-shared seed", Line::Reset { non_shared_seed: 0 }),
            (
                "comment",
                Line::Comment(options.default_comment().to_string()),
            ),
            ("vectorial strafing", {
                let default_option = Line::VectorialStrafing(true);
                if let Some(previous_lines) = previous_lines {
                    if let Some(Line::VectorialStrafing(vectorial_strafing)) = previous_lines
                        .iter()
                        .rev()
                        .find(|line| matches!(line, Line::VectorialStrafing(_)))
                    {
                        Line::VectorialStrafing(!*vectorial_strafing)
                    } else {
                        default_option
                    }
                } else {
                    default_option
                }
            }),
            (
                "vectorial strafing constraints",
                Line::VectorialStrafingConstraints(
                    VectorialStrafingConstraints::VelocityYawLocking { tolerance: 0.0 },
                ),
            ),
            // TODO think about this one
            (
                "change",
                Line::Change(Change {
                    target: ChangeTarget::Yaw,
                    final_value: 0.0,
                    over: 0.4,
                }),
            ),
            ("target yaw override", Line::TargetYawOverride(vec![0.0])),
        ];

        ui.text("new line menu");

        let half_way_index = name_and_types.len() / 2;
        for (i, (button_name, button_type)) in name_and_types.iter().enumerate() {
            if ui.button(button_name) {
                tab.new_line_at_click_index(button_type.to_owned());

                let right_click_index = match tab.tab_menu_data.right_click_popup_index() {
                    Some(index) => index,
                    None => tab.hltas.lines.len() - 1,
                };
                tab.undo_redo_handler.add_lines(vec![right_click_index]);

                ui.close_current_popup();
            }

            if i != half_way_index {
                ui.same_line();
            }
        }
    });

    if tab.hltas_lines_is_empty() {
        if ui.is_mouse_clicked(MouseButton::Right) {
            tab.tab_menu_data.right_click_elsewhere();
            ui.open_popup(new_line_menu_id);
        }

        return;
    }

    let window_y = ui.window_size()[1];

    let mut lines_edited = false;
    let mut stale_line = None;
    let mut new_line_menu_clicked_on_line = false;
    let mut is_modifying_something = false;

    let hltas_lines_is_empty = tab.hltas_lines_is_empty();
    let tab_menu_data = &mut tab.tab_menu_data;
    let properties = &tab.hltas.properties;
    let goto_line = tab_menu_data.goto_line();

    for (i, line) in tab.hltas.lines.iter_mut().enumerate() {
        if let Some(goto_line) = goto_line {
            if i == goto_line {
                ui.set_scroll_here_y_with_ratio(0.0);
            }
        }

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

            // grab area
            // TODO option for area size
            ui.dummy([20.0, 20.0]);
            ui.same_line();
            draw_list
                .add_rect(
                    ui.item_rect_min(),
                    ui.item_rect_max(),
                    ui.style_color(StyleColor::Button),
                )
                .filled(true)
                .build();

            if ui.is_item_clicked()
                && (keyboard_state.held(VirtualKeyCode::LControl)
                    || keyboard_state.held(VirtualKeyCode::RControl))
            {
                tab_menu_data.change_selected_index(i, !tab_menu_data.is_index_selected(i));
            } else if tab_menu_data
                .selected_indexes()
                .iter()
                .filter(|&i| *i)
                .count()
                == 1
                && ui.is_item_clicked()
                && (keyboard_state.held(VirtualKeyCode::LShift)
                    || keyboard_state.held(VirtualKeyCode::RShift))
            {
                let selected_index = tab_menu_data.selected_indexes_collection()[0];
                let (start_index, end_index) = if i < selected_index {
                    (i, selected_index)
                } else {
                    (selected_index, i + 1)
                };

                tab_menu_data.select_index_range(start_index..end_index, true);
            } else if ui.is_item_clicked() {
                let is_selected = tab_menu_data.is_index_selected(i);
                tab_menu_data.reset_selected_indexes();
                tab_menu_data.change_selected_index(i, !is_selected);
            }

            if keyboard_state.just_pressed(VirtualKeyCode::Delete) {}

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
                        let strafe_menu_selection =
                            tab_menu_data.strafe_menu_selection_at_mut(i).unwrap();

                        let strafe_menu_edited = ui.group(|| {
                            show_strafe_menu(ui, strafe_menu_selection, framebulk, &i.to_string())
                        });

                        ui.same_line();
                        ui.set_cursor_screen_pos([jump_menu_offset, ui.cursor_screen_pos()[1]]);

                        // jump menu
                        let jump_menu_edited = ui.group(|| {
                            show_jump_menu(ui, framebulk, properties, &i.to_string(), options)
                        });

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
                        let command_menu_edited = ui.group(|| {
                            show_command_menu(ui, framebulk, &i.to_string(), options.locale_lang())
                        });

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
                        let save_edit_input_edited =
                            InputText::new(ui, format!("##save_edit_input{}", i), save)
                                .chars_noblank(true)
                                .build();
                        save_edit_width.pop(ui);

                        save_edit_input_edited
                    }
                    Line::SharedSeed(shared_seed) => show_shared_seed_editor(
                        ui,
                        ui.window_content_region_width() * 0.25,
                        "properties",
                        shared_seed,
                        options.locale_lang(),
                    ),
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
                    Line::Reset { non_shared_seed } => show_non_shared_seed_editor(
                        ui,
                        ui.window_content_region_width() * 0.25,
                        &format!("##nonshared_seed_edit{}", i),
                        non_shared_seed,
                        options.locale_lang(),
                    ),
                    Line::Comment(comment) => {
                        let comment_frame_bg =
                            ui.push_style_color(StyleColor::FrameBg, [0.0, 0.0, 0.0, 0.0]);
                        let comment_colour =
                            ui.push_style_color(StyleColor::Text, options.comment_colour());

                        let comment_edited =
                            InputText::new(ui, format!("##comment_editor{}", i), comment).build();

                        if ui.is_item_active() {
                            is_modifying_something = true;
                        }

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
                                ("Yaw", ChangeTarget::Yaw),
                                ("Pitch", ChangeTarget::Pitch),
                                ("Target Yaw", ChangeTarget::VectorialStrafingYaw),
                            ],
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
                    // TODO take in calculation for grab area size
                    [min_rect[0] - 20.0 - 25.0, min_rect[1]]
                };

                ui.set_cursor_screen_pos(button_pos);
                if show_x_button(ui, &format!("remove_line_button{}", i)) {
                    line_edited = true;
                    stale_line = Some(i);
                }

                button_color.pop();
                ui.set_cursor_screen_pos(cursor_pos);

                // check if right click for new line menu
                if ui.is_mouse_clicked(MouseButton::Right) {
                    tab_menu_data.set_right_click_index(i + 1);
                    new_line_menu_clicked_on_line = true;
                    ui.open_popup(new_line_menu_id);
                }
            } else if !new_line_menu_clicked_on_line && ui.is_mouse_clicked(MouseButton::Right) {
                if hltas_lines_is_empty {
                    tab_menu_data.right_click_elsewhere();
                } else if i == 0
                    && ui.is_mouse_hovering_rect([0.0, 0.0], [f32::MAX, group_rect_min[1]])
                {
                    // check for right click above first line
                    tab_menu_data.set_right_click_index(i);
                    new_line_menu_clicked_on_line = true;
                } else {
                    tab_menu_data.right_click_elsewhere();
                }
                ui.open_popup(new_line_menu_id);
            }

            draw_list
                .add_rect(
                    group_rect_min,
                    group_rect_max,
                    if tab_menu_data.is_line_selected(i) {
                        [0.678, 0.847, 0.901, 0.2]
                    } else {
                        [0.501, 0.501, 0.501, 0.25]
                    },
                )
                .filled(tab_menu_data.is_line_selected(i))
                .build();

            if !lines_edited && line_edited {
                lines_edited = true;
            }
        } else {
            // TODO find out some better way that doesn't need hardcoded values like this
            ui.dummy([
                0.0,
                match line {
                    Line::FrameBulk(_) => 140.0,
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

    tab_menu_data.set_modifying_something(is_modifying_something);

    if let Some(stale_line) = stale_line {
        tab.undo_redo_handler
            .delete_lines(vec![(stale_line, tab.hltas.lines[stale_line].to_owned())]);
        tab.remove_line_at_index(stale_line);
    }

    if keyboard_state.just_pressed(VirtualKeyCode::Delete)
        || keyboard_state.just_pressed(VirtualKeyCode::Back)
    {
        let lines_to_delete = tab
            .tab_menu_data
            .selected_indexes()
            .iter()
            .enumerate()
            .filter_map(|(i, is_selected)| {
                if *is_selected {
                    Some((i, tab.hltas.lines[i].to_owned()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        tab.undo_redo_handler.delete_lines(lines_to_delete);

        tab.remove_selected_lines();
    }

    if properties_edited || lines_edited {
        tab.tab_menu_data.got_modified();
    }
}
