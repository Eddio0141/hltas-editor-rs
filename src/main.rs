use guis::main::MainGUI;

mod guis;

fn main() {
    let main_gui = MainGUI::default();
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(main_gui), native_options);
}
