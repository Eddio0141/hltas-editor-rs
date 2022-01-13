use hltas::types::{FrameBulk, Properties};
use imgui::Ui;

use crate::guis::main::{
    option_menu::AppOptions, tab::HLTASMenuState, undo_redo_hltas::UndoRedoHandler,
};

pub struct HLTASInfo<'a> {
    pub framebulk: &'a mut FrameBulk,
    pub properties: &'a Properties,
}

impl<'a> HLTASInfo<'a> {
    pub fn new(framebulk: &'a mut FrameBulk, properties: &'a Properties) -> Self {
        Self {
            framebulk,
            properties,
        }
    }
}

pub struct FramebulkEditorMiscData<'a> {
    pub tab_menu_data: &'a mut HLTASMenuState,
    pub options: &'a AppOptions,
    pub undo_redo_handler: &'a mut UndoRedoHandler,
}

impl<'a> FramebulkEditorMiscData<'a> {
    pub fn new(
        tab_menu_data: &'a mut HLTASMenuState,
        options: &'a AppOptions,
        undo_redo_handler: &'a mut UndoRedoHandler,
    ) -> Self {
        Self {
            tab_menu_data,
            options,
            undo_redo_handler,
        }
    }
}

pub trait FramebulkEditor {
    fn show(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool;

    fn show_minimal(
        &self,
        ui: &Ui,
        hltas_info: HLTASInfo,
        framebulk_editor_misc_data: FramebulkEditorMiscData,
        index: usize,
    ) -> bool;
}
