use guis::main::MainGUI;

mod guis;
mod helpers;
mod support;

// TODO move global locale stuff in its own thing
fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        // core_locales: "./locales/core.ftl",
    };
}

fn main() {
    let system = support::init("HLTAS Editor");
    let mut main_gui = MainGUI::default();
    system.main_loop(move |run, ui| main_gui.show(run, ui));
}
