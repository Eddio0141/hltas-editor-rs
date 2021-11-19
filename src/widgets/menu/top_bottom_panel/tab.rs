use std::{num::NonZeroU32, path::PathBuf};

use fluent_templates::{LanguageIdentifier, Loader};
use hltas::{
    types::{Properties, Seeds},
    HLTAS,
};

use crate::helpers::hltas::hltas_to_str;

/// HLTAS but everything editable is string.
/// Only used for graphics editor and related
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug)]
pub struct RawHLTAS {
    pub demo: String,
    pub save: String,
    pub frametime_0ms: String,
    pub shared_rng: Option<u32>,
    pub non_shared_rng: Option<i64>,
    pub hlstrafe_version: String,
    pub load_command: String,
    pub lines: Vec<String>,
}

impl RawHLTAS {
    pub fn from_hltas(hltas: &HLTAS) -> Self {
        // safety check for hltas changes
        let properties = Properties {
            demo: hltas.properties.demo.to_owned(),
            save: hltas.properties.save.to_owned(),
            frametime_0ms: hltas.properties.frametime_0ms.to_owned(),
            seeds: hltas.properties.seeds,
            hlstrafe_version: hltas.properties.hlstrafe_version,
            load_command: hltas.properties.load_command.to_owned(),
        };

        Self {
            demo: match properties.demo {
                Some(demo) => demo,
                None => String::new(),
            },
            save: match properties.save {
                Some(save) => save,
                None => String::new(),
            },
            frametime_0ms: match properties.frametime_0ms {
                Some(frametime_0ms) => frametime_0ms,
                None => String::new(),
            },
            shared_rng: match properties.seeds {
                Some(seeds) => Some(seeds.shared),
                None => None,
            },
            non_shared_rng: match properties.seeds {
                Some(seeds) => Some(seeds.non_shared),
                None => None,
            },
            hlstrafe_version: match properties.hlstrafe_version {
                Some(hlstrafe_version) => hlstrafe_version.to_string(),
                None => String::new(),
            },
            load_command: match properties.load_command {
                Some(load_command) => load_command,
                None => String::new(),
            },
            // HACK better way to do hltas to str
            lines: {
                let hltas_raw = hltas_to_str(&hltas);
                match hltas_raw.find("frames") {
                    Some(frames_index) => hltas_raw[frames_index + 6..]
                        .lines()
                        .map(|s: &str| s.to_string())
                        .collect(),
                    None => Vec::new(),
                }
            },
        }
    }

    pub fn to_hltas(&self) -> HLTAS {
        HLTAS {
            properties: Properties {
                demo: if self.demo == "" {
                    None
                } else {
                    Some(self.demo.to_owned())
                },
                save: if self.save == "" {
                    None
                } else {
                    Some(self.save.to_owned())
                },
                frametime_0ms: if self.frametime_0ms == "" {
                    None
                } else {
                    Some(self.frametime_0ms.to_owned())
                },
                seeds: {
                    let mut seeds = None;
                    if let Some(shared_rng) = self.shared_rng {
                        if let Some(non_shared_rng) = self.non_shared_rng {
                            seeds = Some(Seeds {
                                shared: shared_rng,
                                non_shared: non_shared_rng,
                            })
                        }
                    }
                    seeds
                },
                hlstrafe_version: if self.hlstrafe_version == "" {
                    None
                } else {
                    match self.hlstrafe_version.parse::<NonZeroU32>() {
                        Ok(hlstrafe_version) => Some(hlstrafe_version),
                        Err(_) => None,
                    }
                },
                load_command: if self.load_command == "" {
                    None
                } else {
                    Some(self.load_command.to_owned())
                },
            },
            // HACK better way to do str to lines conversion?
            lines: {
                match HLTAS::from_str(&("version 1\nframes\n".to_owned() + &self.lines.join("\n")))
                {
                    Ok(hltas) => hltas.lines,
                    Err(_) => Vec::new(),
                }
            },
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct HLTASFileTab {
    pub title: String,
    pub path: Option<PathBuf>,
    raw_content: String,
    raw_hltas: RawHLTAS,
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
            hltas: hltas.to_owned(),
            raw_hltas: RawHLTAS::from_hltas(&hltas),
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
            raw_hltas: RawHLTAS::from_hltas(&HLTAS::default()),
        }
        // Self::default()
    }

    // TODO use cache
    pub fn get_raw_content(&self) -> &str {
        &self.raw_content
    }

    pub fn get_raw_hltas(&self) -> &RawHLTAS {
        &self.raw_hltas
    }

    pub fn set_hltas(&mut self, hltas: HLTAS) {
        self.hltas = hltas;
    }

    pub fn set_hltas_from_raw_hltas(&mut self, raw_hltas: &RawHLTAS) {
        self.hltas = raw_hltas.to_hltas();
    }

    pub fn hltas(&self) -> &HLTAS {
        &self.hltas
    }

    // TODO remove this

    /// Get a mutable reference to the hltasfile tab's raw hltas.
    pub fn raw_hltas_mut(&mut self) -> &mut RawHLTAS {
        &mut self.raw_hltas
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
