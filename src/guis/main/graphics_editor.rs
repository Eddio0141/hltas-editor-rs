use std::convert::TryFrom;
use std::num::NonZeroU32;

use super::frametime_changer::frametime_changer;
use super::selectable_hltas_button::selectable_hltas_button;
use super::strafe_selector::strafe_selector;
use super::tab::HLTASFileTab;
use crate::helpers::egui::button::close_button;
use crate::helpers::hltas::frametime;
use eframe::egui::{self, Label};
use eframe::egui::{CollapsingHeader, Color32, DragValue, Id, TextEdit, Ui};

use hltas::types::{Buttons, ChangeTarget, Line, Seeds, VectorialStrafingConstraints};

// TODO preset buttons for common stuff like 1k fps
pub fn show_graphics_editor(ui: &mut Ui, current_tab: &mut HLTASFileTab) {
    egui::ScrollArea::both()
        .max_width(f32::INFINITY)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            // TODO translation?
            let hltas = &mut current_tab.hltas;

            CollapsingHeader::new("properties")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("frametime0ms");
                        // TODO do I even make this an option? maybe make this changable from the gui options
                        ui.set_enabled(false);
                        let set_frametime_button = match &mut hltas.properties.frametime_0ms {
                            Some(frametime) => match frametime.parse::<f32>() {
                                Ok(mut frametime) => {
                                    ui.add(
                                        DragValue::new(&mut frametime)
                                            .speed(frametime::MAX)
                                            .clamp_range(frametime::MIN..=frametime::MAX),
                                    );
                                    hltas.properties.frametime_0ms = Some(frametime.to_string());
                                    if ui.add(close_button().small()).clicked() {
                                        hltas.properties.frametime_0ms = None;
                                    }
                                    false
                                }
                                Err(_) => true,
                            },
                            None => true,
                        };

                        if set_frametime_button {
                            if ui.button("set frametime0ms").clicked() {
                                // TODO implement settings to change this
                                hltas.properties.frametime_0ms = Some("0.0000000001".to_string());
                            }
                        }

                        // TODO remove me when done
                        ui.set_enabled(true);
                        ui.shrink_width_to_current();
                    });

                    ui.horizontal(|ui| {
                        ui.label("seeds");
                        let create_seed_button = match &mut hltas.properties.seeds {
                            Some(seeds) => {
                                let shared_rng = &mut seeds.shared;
                                let nonshared_rng = &mut seeds.non_shared;

                                ui.add(DragValue::new(shared_rng).speed(0.05));
                                ui.add(DragValue::new(nonshared_rng).speed(0.05));
                                if ui.add(close_button().small()).clicked() {
                                    hltas.properties.seeds = None;
                                }
                                false
                            }
                            None => true,
                        };

                        if create_seed_button {
                            if ui.button("set shared non-shared rng").clicked() {
                                hltas.properties.seeds = Some(Seeds {
                                    shared: 0,
                                    non_shared: 0,
                                });
                            }
                        }

                        ui.shrink_width_to_current();
                    });
                });

            ui.separator();
            ui.add(Label::new("Lines").heading());

            // TODO color customization
            // TODO show bulk ID

            // TODO comment enter focus
            // let mut new_comment_insert = None;
            // let focus_mem_id = Id::new("focus_mem");

            for (i, line) in &mut hltas.lines.iter_mut().enumerate() {
                match line {
                    Line::FrameBulk(framebulk) => {
                        ui.horizontal(|ui| {
                            // s03ljDbcgw|flrbud|jdu12r|0.001|180|-89|1|cmd

                            // ui.painter().rect(
                            //     ui.available_rect_before_wrap(), /*.expand(ui.visuals().expansion)*/
                            //     0.3,
                            //     ui.visuals().code_bg_color,
                            //     ui.style().visuals.widgets.inactive.fg_stroke,
                            // );

                            strafe_selector(
                                &mut framebulk.auto_actions.movement,
                                ui,
                                Id::new(format!("strafe_selector_{}", i)),
                            );

                            ui.separator();

                            frametime_changer(&mut framebulk.frame_time, ui);

                            ui.separator();

                            let mut framecount_unwrapped = framebulk.frame_count.get();
                            let framecount_changed = ui
                                .add(
                                    DragValue::new(&mut framecount_unwrapped)
                                        .clamp_range(1..=u32::MAX),
                                )
                                .changed();

                            if framecount_changed {
                                if let Ok(framecount) = NonZeroU32::try_from(framecount_unwrapped) {
                                    framebulk.frame_count = framecount;
                                }
                            }

                            ui.label("frames");
                        });
                    }
                    Line::Save(save) => {
                        ui.label(save);
                    }
                    Line::SharedSeed(shared_seed) => {
                        ui.horizontal(|ui| {
                            // TODO seed changer helper function
                            ui.style_mut().spacing.item_spacing.x = 0.0;

                            ui.label("seed ");
                            // TODO settings for seed drag changer
                            ui.add(DragValue::new(shared_seed).speed(0.05));
                        });
                    }
                    Line::Buttons(buttons) => {
                        ui.horizontal(|ui| {
                            // ui.style_mut().spacing.item_spacing.x = 0.0;
                            ui.label("Buttons");

                            match buttons {
                                Buttons::Reset => {
                                    ui.label("reset");
                                }
                                Buttons::Set {
                                    air_left,
                                    air_right,
                                    ground_left,
                                    ground_right,
                                } => {
                                    ui.separator();
                                    ui.label("air left");
                                    selectable_hltas_button(air_left, ui, Id::new("air_left"));
                                    ui.label("air right");
                                    selectable_hltas_button(air_right, ui, Id::new("air_right"));
                                    ui.label("ground left");
                                    selectable_hltas_button(
                                        ground_left,
                                        ui,
                                        Id::new("ground_left"),
                                    );
                                    ui.label("ground right");
                                    selectable_hltas_button(
                                        ground_right,
                                        ui,
                                        Id::new("ground_right"),
                                    );
                                }
                            };
                        });
                    }
                    Line::LGAGSTMinSpeed(lgagstminspd) => {
                        ui.horizontal(|ui| {
                            ui.label("lgagst minimum speed ");
                            ui.add(
                                DragValue::new(lgagstminspd)
                                    .speed(0.05)
                                    .clamp_range(0.0..=f32::INFINITY),
                            );
                        });
                    }
                    Line::Reset { non_shared_seed } => {
                        ui.horizontal(|ui| {
                            // TODO seed changer helper function
                            ui.style_mut().spacing.item_spacing.x = 0.0;

                            ui.label("reset ");
                            // TODO settings for seed drag changer
                            ui.add(DragValue::new(non_shared_seed).speed(0.05));
                        });
                    }
                    Line::Comment(comment) => {
                        ui.horizontal(|ui| {
                            ui.style_mut().spacing.item_spacing.x = 0.0;
                            let comment_color = Color32::from_rgb(0, 255, 0);
                            ui.colored_label(comment_color, "//");
                            ui.add(
                                TextEdit::singleline(comment)
                                    .text_color(comment_color)
                                    .desired_width(f32::INFINITY)
                                    .frame(false),
                            );

                            // let focus_comment = if let Some(index) = ui
                            //     .memory()
                            //     .id_data_temp
                            //     .get_or_insert_with(focus_mem_id, || {
                            //         Option::<usize>::None
                            //     }) {
                            //     *index == i
                            // } else {
                            //     false
                            // };

                            // let is_focused = {
                            //     let response = ui.add(
                            //         TextEdit::singleline(comment)
                            //             .text_color(comment_color)
                            //             .desired_width(f32::INFINITY)
                            //             .frame(false),
                            //     );
                            //     if focus_comment {
                            //         response.request_focus();
                            //         ui.memory()
                            //             .id_data_temp
                            //             .insert(focus_mem_id, || Option::<usize>::None);

                            //         true
                            //     } else {
                            //         response.lost_focus()
                            //     }
                            // };

                            // if is_focused && ui.input().key_pressed(Key::Enter) {
                            //     new_comment_insert = Some(i + 1);

                            //     ui.memory().id_data_temp.insert(focus_mem_id, Some(i + 1));
                            // }
                        });
                    }
                    Line::VectorialStrafing(vectorial_strafing) => {
                        ui.checkbox(vectorial_strafing, "vectorial strafing");
                    }
                    Line::VectorialStrafingConstraints(vectorial_strafing_constraints) => {
                        ui.horizontal(|ui| {
                            let target_yaw_colour = Color32::from_rgb(255, 0, 0);
                            match vectorial_strafing_constraints {
                                VectorialStrafingConstraints::VelocityYaw { tolerance } => {
                                    ui.colored_label(target_yaw_colour, "target yaw velocity");
                                    // TODO idk the limit
                                    ui.add(
                                        DragValue::new(tolerance)
                                            .speed(0.05)
                                            .clamp_range(0.0..=360.0),
                                    );
                                }
                                VectorialStrafingConstraints::AvgVelocityYaw { tolerance } => {
                                    ui.colored_label(
                                        target_yaw_colour,
                                        "target yaw velocity avg +-",
                                    );
                                    // TODO idk the limit
                                    ui.add(
                                        DragValue::new(tolerance)
                                            .speed(0.05)
                                            .clamp_range(0.0..=360.0),
                                    );
                                }
                                VectorialStrafingConstraints::VelocityYawLocking { tolerance } => {
                                    ui.horizontal(|ui| {
                                        ui.colored_label(
                                            target_yaw_colour,
                                            "target_yaw velocity_lock",
                                        );
                                        ui.add(
                                            DragValue::new(tolerance)
                                                .speed(0.05)
                                                .clamp_range(0.0..=360.0),
                                        );
                                    });
                                }
                                VectorialStrafingConstraints::Yaw { yaw, tolerance } => {
                                    ui.colored_label(target_yaw_colour, "target yaw ");
                                    ui.add(
                                        DragValue::new(yaw).speed(0.05).clamp_range(0.0..=360.0),
                                    );
                                    ui.colored_label(target_yaw_colour, "+-");
                                    ui.add(
                                        DragValue::new(tolerance)
                                            .speed(0.05)
                                            .clamp_range(0.0..=360.0),
                                    );
                                }
                                VectorialStrafingConstraints::YawRange { from, to } => {
                                    ui.label("target_yaw from");
                                    ui.add(
                                        DragValue::new(from).speed(0.05).clamp_range(0.0..=360.0),
                                    );
                                    ui.label("to");
                                    ui.add(DragValue::new(to).speed(0.05).clamp_range(0.0..=360.0));
                                }
                            }
                        });
                    }
                    Line::Change(change) => {
                        ui.horizontal(|ui| {
                            let target_text = match change.target {
                                ChangeTarget::Yaw => "yaw",
                                ChangeTarget::Pitch => "pitch",
                                ChangeTarget::VectorialStrafingYaw => "target_yaw",
                            };

                            ui.label(format!("change {} to", target_text));
                            ui.add(
                                DragValue::new(&mut change.final_value)
                                    .speed(0.05)
                                    .clamp_range(0.0..=360.0),
                            );
                            ui.label("over");
                            ui.add(
                                DragValue::new(&mut change.over)
                                    .speed(0.01)
                                    .clamp_range(0.0..=f32::INFINITY),
                            );
                            ui.label("s");
                        });
                    }
                    // TODO implement target_yaw_override
                    Line::TargetYawOverride(_target_yaw) => {
                        ui.label("target_yaw_override...");
                    }
                };
            }

            // TODO comment focus thing
            // if let Some(insert_index) = new_comment_insert {
            //     hltas
            //         .lines
            //         .insert(insert_index, Line::Comment(String::new()));
            // }
        });
}
