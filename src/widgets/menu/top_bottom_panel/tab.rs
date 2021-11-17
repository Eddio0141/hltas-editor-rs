use std::path::PathBuf;

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::HLTAS;

use crate::helpers::{self, hltas::hltas_to_str};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct HLTASFileTab {
    pub title: String,
    pub path: Option<PathBuf>,
    raw_content: String,
    // TODO implement serialization
    #[cfg_attr(feature = "persistence", serde(skip))]
    hltas: HLTAS,
    pub got_modified: bool,
}

impl<'a> HLTASFileTab {
    pub fn open_path(
        path: &PathBuf,
        file_content: &'a str,
    ) -> Result<Self, hltas::read::Error<'a>> {
        let hltas = match HLTAS::from_str(&file_content) {
            Ok(hltas) => hltas,
            Err(err) => return Err(err),
        };

        Ok(Self {
            // TODO error check?
            // this is file so it should be
            title: path.file_name().unwrap().to_str().unwrap().to_owned(),
            path: Some(path.clone()),
            raw_content: file_content.to_string(),
            // ..Default::default()
            got_modified: false,
            hltas,
        })
    }

    pub fn title_from_path(path: &PathBuf, lang: &LanguageIdentifier) -> String {
        if let Some(os_str) = path.file_name() {
            if let Some(str) = os_str.to_str() {
                return str.to_owned();
            }
        }
        HLTASFileTab::default_title(lang).to_owned()
    }

    // BUG fix language change for title (opt out serialization for the titles?)
    fn default_title(lang: &LanguageIdentifier) -> String {
        crate::LOCALES.lookup(lang, "new-file-title")
    }

    pub fn new_file(lang: &LanguageIdentifier) -> Self {
        // TODO maybe make the language global?
        Self {
            title: Self::default_title(lang).to_string(),
            path: None,
            raw_content: hltas_to_str(&HLTAS::default()),
            got_modified: false,
            hltas: HLTAS::default(),
        }
        // Self::default()
    }

    // TODO use cache
    pub fn raw_content(&self) -> &str {
        &self.raw_content
    }

    pub fn set_hltas(&mut self, hltas: HLTAS) {
        self.raw_content = helpers::hltas::hltas_to_str(&self.hltas);
        self.hltas = hltas;
    }

    pub fn hltas(&self) -> &HLTAS {
        &self.hltas
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
