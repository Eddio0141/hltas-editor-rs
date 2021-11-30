use std::{
    ops::Deref,
    path::{Path, PathBuf},
    slice::Iter,
};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::HLTAS;

pub struct Tabs<T> {
    tabs: Vec<T>,
    current_tab_index: Option<usize>,
}

impl<T> Tabs<T> {
    pub fn new(tabs: Vec<T>) -> Self {
        let current_tab_index = if tabs.len() > 0 {
            Some(tabs.len() - 1)
        } else {
            None
        };

        Self {
            tabs,
            current_tab_index,
        }
    }

    pub fn current_tab(&self) -> Option<&T> {
        match self.current_tab_index {
            Some(index) => Some(&self.tabs[index]),
            None => None,
        }
    }

    pub fn current_tab_mut(&mut self) -> Option<&mut T> {
        match self.current_tab_index {
            Some(index) => Some(&mut self.tabs[index]),
            None => None,
        }
    }

    pub fn remove_current_tab(&mut self) {
        if let Some(index) = self.current_tab_index {
            self.tabs.remove(index);
        }
        if self.tabs.len() == 0 {
            self.current_tab_index = None;
        } else if let Some(index) = self.current_tab_index {
            if index >= self.tabs.len() {
                self.current_tab_index = Some(self.tabs.len() - 1);
            }
        }
    }

    pub fn tabs(&self) -> &Vec<T> {
        &self.tabs
    }

    pub fn push_tab(&mut self, tab: T) {
        self.tabs.push(tab);

        self.current_tab_index = Some(self.tabs.len() - 1);
    }

    pub fn iter(&self) -> Iter<T> {
        self.tabs.iter()
    }

    pub fn set_current_tab(&mut self, index: Option<usize>) {
        // intentionally fix it so it doesn't go out of index
        // not sure if I should make this panic instead
        let index = if self.tabs.len() == 0 {
            None
        } else if let Some(index) = index {
            if index >= self.tabs.len() {
                Some(self.tabs.len())
            } else {
                Some(index)
            }
        } else {
            None
        };

        self.current_tab_index = index;
    }
}

pub struct HLTASFileTab {
    pub title: String,
    pub path: Option<PathBuf>,
    pub hltas: HLTAS,
    pub got_modified: bool,
}

// TODO think if pathbuf can be a generic type
impl<'a> HLTASFileTab {
    pub fn open_path(path: &Path, file_content: &'a str) -> Result<Self, hltas::read::Error<'a>> {
        let hltas = match HLTAS::from_str(&file_content) {
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

        Ok(Self {
            title,
            path: Some(path.to_path_buf()),
            got_modified: false,
            hltas,
        })
    }

    pub fn title_from_path<'b>(path: &'b PathBuf, lang: &'b LanguageIdentifier) -> String {
        if let Some(os_str) = path.file_name() {
            if let Some(str) = os_str.to_str() {
                // TODO replace this?
                return str.to_owned();
            }
        }
        HLTASFileTab::default_title(&lang)
    }

    // BUG fix language change for title (opt out serialization for the titles?)
    fn default_title(lang: &LanguageIdentifier) -> String {
        crate::LOCALES.lookup(&lang, "new-file-title")
    }

    pub fn new_file(lang: &LanguageIdentifier /*, file_value: usize*/) -> Self {
        // TODO maybe make the language global?
        Self {
            // title: format!("{} {}", Self::default_title(lang), file_value),
            title: Self::default_title(lang).to_string(),
            path: None,
            got_modified: false,
            hltas: HLTAS::default(),
        }
        // Self::default()
    }
}
