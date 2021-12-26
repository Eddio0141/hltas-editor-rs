use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::{
    types::{AutoMovement, FrameBulk, Line},
    HLTAS,
};

use crate::locale::LOCALES;

pub struct HLTASFileTab {
    pub title: String,
    pub path: Option<PathBuf>,
    pub hltas: HLTAS,
    pub got_modified: bool,

    pub tab_menu_data: HLTASMenuState,
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
            got_modified: false,
            hltas,
            tab_menu_data,
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

    // BUG fix language change for title (opt out serialization for the titles?)
    fn default_title(lang: &LanguageIdentifier) -> String {
        LOCALES.lookup(lang, "new-file-title")
    }

    pub fn new_file(lang: &LanguageIdentifier /*, file_value: usize*/) -> Self {
        // TODO maybe make the language global?
        Self {
            // title: format!("{} {}", Self::default_title(lang), file_value),
            title: Self::default_title(lang),
            path: None,
            got_modified: false,
            hltas: HLTAS::default(),
            tab_menu_data: HLTASMenuState::new(&HLTAS::default()),
        }
        // Self::default()
    }
}

/// Struct to keep track of some menu states for the hltas object in the tab
pub struct HLTASMenuState {
    pub strafe_menu_selections: Vec<Option<StrafeMenuSelection>>,
    pub right_click_popup_index: Option<usize>,
    pub selected_indexes: Vec<Option<()>>,
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
            strafe_menu_selections,
            right_click_popup_index: None,
            selected_indexes: vec![None; hltas.lines.len()],
        }
    }

    /// Get a reference to the hltasmenu state's selected indexes.
    pub fn selected_indexes(&self) -> &[Option<()>] {
        self.selected_indexes.as_ref()
    }

    pub fn reset_selected_indexes(&mut self) {
        self.selected_indexes = vec![None; self.selected_indexes.len()];
    }

    pub fn select_all_indexes(&mut self) {
        self.selected_indexes = vec![Some(()); self.selected_indexes.len()];
    }
}

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
