use eframe::egui::{Align, Area, Frame, Id, Key, Layout, Order, Response, Ui};

pub fn popup_to_widget_right<R>(
    ui: &Ui,
    popup_id: Id,
    widget_response: &Response,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    if ui.memory().is_popup_open(popup_id) {
        let parent_clip_rect = ui.clip_rect();
        let mut pos = widget_response.rect.right_top();
        pos.x -= 15.0;

        let inner = Area::new(popup_id)
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(parent_clip_rect); // for when the combo-box is in a scroll area.
                let frame = Frame::popup(ui.style());
                frame
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                            // ui.set_width(widget_response.rect.width() - 2.0 * frame_margin.x);
                            ui.set_width(0.0);
                            add_contents(ui)
                        })
                        .inner
                    })
                    .inner
            })
            .inner;

        if ui.input().key_pressed(Key::Escape) || widget_response.clicked_elsewhere() {
            ui.memory().close_popup();
        }
        Some(inner)
    } else {
        None
    }
}
