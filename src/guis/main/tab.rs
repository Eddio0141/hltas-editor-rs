use std::{ops::Deref, path::{Path, PathBuf}};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::HLTAS;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct HLTASFileTab {
    pub title: String,
    pub path: Option<PathBuf>,
    // TODO implement serialization
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub hltas: HLTAS,
    pub got_modified: bool,
}

// TODO think if pathbuf can be a generic type
impl<'a> HLTASFileTab {
    pub fn open_path(path: &Path, file_content: &'a str) -> Result<Self, hltas::read::Error<'a>>
    {
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
                return str.to_owned();
            }
        }
        HLTASFileTab::default_title(&lang)
    }

    // BUG fix language change for title (opt out serialization for the titles?)
    fn default_title(lang: &LanguageIdentifier) -> String {
        crate::LOCALES.lookup(&lang, "new-file-title")
    }

    pub fn new_file(lang: &LanguageIdentifier) -> Self {
        // TODO maybe make the language global?
        Self {
            title: Self::default_title(lang).to_string(),
            path: None,
            got_modified: false,
            hltas: HLTAS::default(),
        }
        // Self::default()
    }
}

// impl Default for Tab {
//     fn default() -> Self {
//         Self {
//             title: Tab::default_title().to_owned(),
//             path: None,
//             raw_content: hltas_to_str(&HLTAS::default()),
//             got_modified: false,
//         }
//     }
// }
