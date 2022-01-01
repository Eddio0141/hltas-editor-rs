use imgui::{Drag, Ui};

use crate::helpers::locale::locale_lang::LocaleLang;

pub fn show_non_shared_seed_editor(
    ui: &Ui,
    width: f32,
    id: &str,
    non_shared_seed: &mut i64,
    locale_lang: &LocaleLang,
) -> bool {
    let drag_width_token = ui.push_item_width(width * 0.8);
    let seed_edited = Drag::new(format!(
        "{}##{}non_shared_rng_drag_edit",
        locale_lang.get_string_from_id("non-shared-rng"),
        id
    ))
    .speed(0.05)
    .build(ui, non_shared_seed);
    drag_width_token.pop(ui);
    ui.same_line();
    let add_sub_width_token = ui.push_item_width(width * 0.08);
    ui.same_line();
    let seed_added = ui.button(format!("+##{}nonshared_seed_add", id));
    ui.same_line();
    let seed_subtracted = ui.button(format!("-##{}nonshared_seed_subtract", id));
    add_sub_width_token.pop(ui);

    if seed_added {
        *non_shared_seed += 1;
    }
    if seed_subtracted {
        *non_shared_seed -= 1;
    }

    seed_edited || seed_added || seed_subtracted
}

pub fn show_shared_seed_editor(
    ui: &Ui,
    width: f32,
    id: &str,
    shared_seed: &mut u32,
    locale_lang: &LocaleLang,
) -> bool {
    let drag_width_token = ui.push_item_width(width * 0.8);
    let seed_edited = Drag::new(format!(
        "{}##{}shared_rng_drag_edit",
        locale_lang.get_string_from_id("shared-rng"),
        id
    ))
    .speed(0.05)
    .build(ui, shared_seed);
    drag_width_token.pop(ui);
    ui.same_line();
    let add_sub_width_token = ui.push_item_width(width * 0.08);
    ui.same_line();
    let seed_added = ui.button(format!("+##{}shared_seed_add", id));
    ui.same_line();
    let seed_subtracted = ui.button(format!("-##{}shared_seed_subtract", id));
    add_sub_width_token.pop(ui);

    ui.same_line();
    ui.text(format!("(mod 256 = {})", *shared_seed % 256));

    if seed_added {
        *shared_seed += 1;
    }
    if seed_subtracted && *shared_seed > 0 {
        *shared_seed -= 1;
    }

    seed_edited || seed_added || seed_subtracted
}
