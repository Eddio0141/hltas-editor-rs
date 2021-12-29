use imgui::{Ui, SliderFlags, Drag};

pub fn show_zero_ms_editor(ui: &Ui, id: &str, frametime: &mut f32) -> bool {
    Drag::new(format!("##0ms_frametime_editor{}", id))
        .range(f32::MIN_POSITIVE, 0.0009)
        .flags(SliderFlags::LOGARITHMIC)
        .speed(0.0000001)
        .display_format("%.38f")
        .build(ui, frametime)
}
