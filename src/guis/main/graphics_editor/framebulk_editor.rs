use hltas::types::{FrameBulk, Properties};
use imgui::Ui;

use crate::guis::main::{
    option_menu::AppOptions, tab::HLTASMenuState, undo_redo_hltas::UndoRedoHandler,
};

pub trait FramebulkEditor {
    fn show(
        &self,
        ui: &Ui,
        framebulk: &mut FrameBulk,
        properties: &Properties,
        tab_menu_data: &mut HLTASMenuState,
        options: &AppOptions,
        undo_redo_handler: &mut UndoRedoHandler,
        index: usize,
    ) -> bool;

    fn show_minimal(
        &self,
        ui: &Ui,
        framebulk: &mut FrameBulk,
        properties: &Properties,
        tab_menu_data: &mut HLTASMenuState,
        options: &AppOptions,
        undo_redo_handler: &mut UndoRedoHandler,
        index: usize,
    ) -> bool;
}
