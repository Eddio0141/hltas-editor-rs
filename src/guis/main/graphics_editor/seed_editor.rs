use imgui::{Drag, Ui};

pub fn show_non_shared_seed_editor(
    ui: &Ui,
    width: f32,
    id: &str,
    non_shared_seed: &mut i64,
) -> bool {
    let drag_width_token = ui.push_item_width(width * 0.8);
    let seed_edited = Drag::new(format!("non-shared rng##{}drag_edit", id))
        .speed(0.05)
        .build(ui, non_shared_seed);
    drag_width_token.pop(ui);
    ui.same_line();
    let add_sub_width_token = ui.push_item_width(width * 0.08);
    ui.same_line();
    let seed_added = ui.button(format!("+##{}seed_add", id));
    ui.same_line();
    let seed_subtracted = ui.button(format!("-##{}seed_subtract", id));
    add_sub_width_token.pop(ui);

    if seed_added {
        *non_shared_seed += 1;
    }
    if seed_subtracted {
        *non_shared_seed -= 1;
    }

    seed_edited || seed_added || seed_subtracted
}
