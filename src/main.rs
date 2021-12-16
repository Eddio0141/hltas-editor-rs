use guis::main::MainGUI;

mod guis;
mod helpers;
mod locale;
mod support;

fn main() {
    let system = support::init("HLTAS Editor");
    let mut main_gui = MainGUI::default();
    system.main_loop(move |run, ui| main_gui.show(run, ui));
}
