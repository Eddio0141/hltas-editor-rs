use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::HLTAS;

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
