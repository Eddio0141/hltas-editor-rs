use eframe::egui::Ui;

use super::tab::HLTASFileTab;

pub fn show_text_editor(_ui: &mut Ui, _current_tab: &mut HLTASFileTab) {
    // let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
    //     let mut layout_job: egui::text::LayoutJob = my_memoized_highlighter(string);
    //     layout_job.wrap_width = wrap_width;
    //     ui.fonts().layout_job(layout_job)
    // };
    // ui.add(egui::TextEdit::multiline(&mut my_code).layouter(&mut layouter));

    // TODO show line count
    // ScrollArea::both().show(ui, |ui| {
    //     // HACK find a better method
    //     let mut raw_hltas = current_tab.get_raw_content().to_owned();
    //     let tab_content_changed = ui
    //         .add(
    //             TextEdit::multiline(&mut raw_hltas)
    //                 .text_style(TextStyle::Monospace)
    //                 .code_editor()
    //                 .desired_rows(1)
    //                 .lock_focus(true)
    //                 .desired_width(f32::INFINITY), // .layouter(&mut layouter)
    //         )
    //         .changed();

    //     if tab_content_changed {
    //         if let Ok(hltas) = HLTAS::from_str(&raw_hltas) {
    //             current_tab.set_hltas(hltas);
    //         }
    //     }
    //     if tab_content_changed {
    //         current_tab.got_modified = true;
    //     }
    // });
}
