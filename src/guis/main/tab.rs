use std::{
    fs,
    ops::{Deref, Range},
    path::{Path, PathBuf},
};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::{
    types::{AutoMovement, FrameBulk, Line, Properties},
    HLTAS,
};
use hltas_cleaner::CleanerResult;
use native_dialog::FileDialog;

use crate::{
    helpers::{hltas::hltas_to_str, locale::locale_lang::LocaleLang},
    locale::LOCALES,
};

use super::undo_redo_hltas::UndoRedoHandler;

#[derive(Clone, Debug, Default)]
pub struct HLTASFileTab {
    title: String,
    path: Option<PathBuf>,
    hltas: HLTAS,
    pub tab_menu_data: HLTASMenuState,
    pub undo_redo_handler: UndoRedoHandler,
}

impl<'a> HLTASFileTab {
    pub fn open_path(path: &Path, file_content: &'a str) -> Result<Self, hltas::read::Error<'a>> {
        let hltas = match HLTAS::from_str(file_content) {
            Ok(hltas) => hltas,
            Err(err) => return Err(err),
        };

        let title = {
            let path_name = path.file_name().unwrap();
            match path_name.to_str() {
                Some(str_name) => str_name.to_owned(),
                None => path_name.to_string_lossy().deref().to_owned(),
            }
        };

        let tab_menu_data = HLTASMenuState::new(&hltas);

        Ok(Self {
            title,
            path: Some(path.to_path_buf()),
            hltas,
            tab_menu_data,
            ..Default::default()
        })
    }

    pub fn title_from_path(path: &Path, lang: &LanguageIdentifier) -> String {
        if let Some(os_str) = path.file_name() {
            if let Some(str) = os_str.to_str() {
                return str.to_owned();
            }
        }
        HLTASFileTab::default_title(lang)
    }

    // BUG fix language change for title
    fn default_title(lang: &LanguageIdentifier) -> String {
        LOCALES.lookup(lang, "new-file-title")
    }

    pub fn new_file(lang: &LanguageIdentifier) -> Self {
        Self {
            title: Self::default_title(lang),
            ..Default::default()
        }
    }

    pub fn hltas_properties_mut(&mut self) -> &mut Properties {
        &mut self.hltas.properties
    }

    pub fn hltas_lines(&self) -> &[Line] {
        &self.hltas.lines
    }

    pub fn split_fields_mut(
        &mut self,
    ) -> (
        &mut [Line],
        &mut Properties,
        &mut HLTASMenuState,
        &mut UndoRedoHandler,
    ) {
        (
            &mut self.hltas.lines,
            &mut self.hltas.properties,
            &mut self.tab_menu_data,
            &mut self.undo_redo_handler,
        )
    }

    pub fn insert_line(&mut self, index: usize, line: hltas::types::Line) {
        self.tab_menu_data.insert_hltas_line(index, &line);
        self.hltas.lines.insert(index, line);
        self.tab_menu_data.got_modified();
    }

    pub fn push_line(&mut self, line: hltas::types::Line) {
        self.tab_menu_data.push_hltas_line(&line);
        self.hltas.lines.push(line);
        self.tab_menu_data.got_modified();
    }

    pub fn hltas_cleaner_fn<F>(&mut self, cleaner: F)
    where
        F: FnOnce(&mut HLTAS) -> CleanerResult,
    {
        let cleaner_result = cleaner(&mut self.hltas);

        for i in cleaner_result.lines_removed.iter().rev() {
            self.tab_menu_data.remove_line_at_index(*i);
        }

        self.tab_menu_data.got_modified();
    }

    pub fn new_line_at_click_index(&mut self, line: hltas::types::Line) {
        match self.tab_menu_data.right_click_popup_index() {
            Some(index) => self.insert_line(index, line),
            None => self.push_line(line),
        }
        self.tab_menu_data.got_modified();
    }

    pub fn remove_line_at_index(&mut self, index: usize) {
        self.hltas.lines.remove(index);
        self.tab_menu_data.remove_line_at_index(index);
        self.tab_menu_data.got_modified();
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    fn ask_hltas_save_location() -> Result<Option<PathBuf>, native_dialog::Error> {
        FileDialog::new()
            .add_filter("HLTAS Files", &["hltas"])
            .show_save_single_file()
    }

    pub fn write_hltas_to_file(&mut self, locale_lang: &LocaleLang) -> Result<(), std::io::Error> {
        if let Some(path) = &self.path {
            // save_path = Some(path.to_owned());
            fs::write(path, hltas_to_str(&self.hltas))?;
            self.tab_menu_data.saved_modified();
        } else {
            // no file, save as new file
            if let Ok(Some(path)) = Self::ask_hltas_save_location() {
                fs::write(&path, hltas_to_str(&self.hltas))?;
                self.title = Self::title_from_path(&path, &locale_lang.get_lang());
            }
        }

        Ok(())
    }

    pub fn select_all_lines(&mut self) {
        self.tab_menu_data.select_all_indexes();
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn get_selected_lines(&self) -> Vec<&Line> {
        self.hltas
            .lines
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if self.tab_menu_data.is_index_selected(i) {
                    Some(line)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn remove_selected_lines(&mut self) {
        for index in self
            .tab_menu_data
            .selected_indexes_collection()
            .iter()
            .rev()
        {
            self.remove_line_at_index(*index);
        }
    }

    pub fn undo_hltas(&mut self) {
        self.undo_redo_handler
            .undo(&mut self.hltas, &mut self.tab_menu_data);
    }

    pub fn redo_hltas(&mut self) {
        self.undo_redo_handler
            .redo(&mut self.hltas, &mut self.tab_menu_data);
    }
}

/// Struct to keep track of some menu states for the hltas object in the tab
#[derive(Clone, Debug, Default)]
pub struct HLTASMenuState {
    strafe_menu_selections: Vec<Option<StrafeMenuSelection>>,
    simple_view_show_fps: bool,
    right_click_popup_index: Option<usize>,
    selected_indexes: Vec<bool>,
    hltas_modified: bool,
    goto_line: Option<usize>,
    simple_view: bool,
    is_modifying_line: bool,
    line_edit_backup: Option<(Line, usize)>,
}

impl HLTASMenuState {
    pub fn new(hltas: &HLTAS) -> Self {
        let strafe_menu_selections = hltas
            .lines
            .iter()
            .map(|framebulk| {
                if let Line::FrameBulk(framebulk) = framebulk {
                    Some(StrafeMenuSelection::new(framebulk))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Self {
            line_edit_backup: None,
            simple_view_show_fps: true,
            is_modifying_line: false,
            strafe_menu_selections,
            right_click_popup_index: None,
            selected_indexes: vec![false; hltas.lines.len()],
            hltas_modified: false,
            goto_line: None,
            simple_view: false,
        }
    }

    /// Called before rendering / modifying the lines
    pub fn tick(&mut self) {
        self.is_modifying_line = false;
    }

    pub fn is_modifying_line(&self) -> bool {
        self.is_modifying_line
    }

    pub fn set_modifying_line(&mut self) {
        self.is_modifying_line = true;
    }

    pub fn set_framebulk_edit_backup(&mut self, framebulk: &FrameBulk, index: usize) {
        self.line_edit_backup = Some((Line::FrameBulk(framebulk.to_owned()), index));
    }

    // pub fn set_line_edit_backup(&mut self, line: &Line, index: usize) {
    //     self.line_edit_backup = Some((line.to_owned(), index));
    // }

    pub fn set_undo_point_with_backup(&mut self, undo_redo_handler: &mut UndoRedoHandler) {
        if let Some((line, index)) = &self.line_edit_backup {
            undo_redo_handler.edit_line(line.to_owned(), *index);
            self.line_edit_backup = None;
        }
    }

    // pub fn enable_simple_view(&mut self) {
    //     self.simple_view = true;
    // }

    // pub fn disable_simple_view(&mut self) {
    //     self.simple_view = false;
    // }

    pub fn toggle_simple_view(&mut self) {
        self.simple_view = !self.simple_view;
    }

    pub fn is_index_selected(&self, index: usize) -> bool {
        self.selected_indexes[index]
    }

    /// Get a reference to the hltasmenu state's selected indexes.
    pub fn selected_indexes(&self) -> &[bool] {
        self.selected_indexes.as_ref()
    }

    pub fn selected_indexes_collection(&self) -> Vec<usize> {
        self.selected_indexes
            .iter()
            .enumerate()
            .filter_map(|(i, is_selected)| if *is_selected { Some(i) } else { None })
            .collect::<Vec<_>>()
    }

    pub fn reset_selected_indexes(&mut self) {
        self.selected_indexes = vec![false; self.selected_indexes.len()];
    }

    pub fn select_all_indexes(&mut self) {
        self.selected_indexes = vec![true; self.selected_indexes.len()];
    }

    pub fn change_selected_index(&mut self, index: usize, state: bool) {
        self.selected_indexes[index] = state;
    }

    pub fn select_index_range(&mut self, range: Range<usize>, state: bool) {
        self.selected_indexes[range]
            .iter_mut()
            .for_each(|index| *index = state);
    }

    pub fn insert_hltas_line(&mut self, index: usize, line: &hltas::types::Line) {
        self.strafe_menu_selections.insert(
            index,
            match line {
                Line::FrameBulk(framebulk) => Some(StrafeMenuSelection::new(framebulk)),
                _ => None,
            },
        );
        self.selected_indexes.insert(index, false);
    }

    pub fn push_hltas_line(&mut self, line: &hltas::types::Line) {
        self.strafe_menu_selections.push(match line {
            Line::FrameBulk(framebulk) => Some(StrafeMenuSelection::new(framebulk)),
            _ => None,
        });
        self.selected_indexes.push(false);
    }

    pub fn remove_line_at_index(&mut self, index: usize) {
        self.strafe_menu_selections.remove(index);
        self.selected_indexes.remove(index);
    }

    pub fn set_right_click_index(&mut self, index: usize) {
        self.right_click_popup_index = Some(index);
    }

    pub fn right_click_elsewhere(&mut self) {
        self.right_click_popup_index = None;
    }

    /// Get a reference to the hltasmenu state's right click popup index.
    pub fn right_click_popup_index(&self) -> Option<usize> {
        self.right_click_popup_index
    }

    pub fn got_modified(&mut self) {
        self.hltas_modified = true;
    }

    pub fn saved_modified(&mut self) {
        self.hltas_modified = false;
    }

    pub fn is_modified(&self) -> bool {
        self.hltas_modified
    }

    pub fn strafe_menu_selection_at_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Option<StrafeMenuSelection>> {
        self.strafe_menu_selections.get_mut(index)
    }

    pub fn is_line_selected(&self, index: usize) -> bool {
        self.selected_indexes()[index]
    }

    /// Gets goto line and sets itself to None
    pub fn goto_line(&mut self) -> Option<usize> {
        let goto = self.goto_line;
        self.goto_line = None;
        goto
    }

    pub fn set_goto_line(&mut self, index: usize) {
        self.goto_line = Some(index);
    }

    pub fn simple_view(&self) -> bool {
        self.simple_view
    }

    pub fn simple_view_show_fps(&self) -> bool {
        self.simple_view_show_fps
    }

    pub fn set_simple_view_show_fps(&mut self, simple_view_show_fps: bool) {
        self.simple_view_show_fps = simple_view_show_fps;
    }
}

#[derive(Clone, Debug)]
pub enum StrafeMenuSelection {
    Strafe,
    Keys,
}

impl StrafeMenuSelection {
    pub fn new(framebulk: &FrameBulk) -> Self {
        if let Some(AutoMovement::Strafe(_)) = framebulk.auto_actions.movement {
            StrafeMenuSelection::Strafe
        } else {
            let movement_keys = &framebulk.movement_keys;
            if movement_keys.down
                || movement_keys.up
                || movement_keys.forward
                || movement_keys.left
                || movement_keys.right
                || movement_keys.back
            {
                StrafeMenuSelection::Keys
            } else {
                StrafeMenuSelection::Strafe
            }
        }
    }
}
