use fluent_templates::{LanguageIdentifier, Loader};
use locale_config::Locale;
use serde::Serialize;

use crate::locale::LOCALES;

fn get_fallback_lang() -> LanguageIdentifier {
    "en-US".parse::<LanguageIdentifier>().unwrap()
}

#[derive(Clone, Default)]
pub struct LocaleLang {
    lang: Option<LanguageIdentifier>,
}

impl LocaleLang {
    pub fn new(lang: Option<LanguageIdentifier>) -> Self {
        Self { lang }
    }

    // TODO cache
    pub fn get_lang(&self) -> LanguageIdentifier {
        match &self.lang {
            Some(lang) => lang.to_owned(),
            // shouldn't error
            None => Locale::user_default()
                .to_string()
                .split(',')
                .collect::<Vec<_>>()[0]
                .parse()
                .unwrap_or_else(|_| get_fallback_lang()),
        }
    }

    pub fn set_lang(&mut self, lang: &LanguageIdentifier) {
        self.lang = Some(lang.to_owned());
    }

    pub fn use_system_lang(&mut self) {
        self.lang = None;
    }

    pub fn is_using_system_lang(&self) -> bool {
        self.lang.is_none()
    }

    pub fn get_string_from_id(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.get_lang(), text_id)
    }
}

impl Serialize for LocaleLang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.lang {
            Some(lang) => serializer.serialize_str(&lang.to_string()),
            None => serializer.serialize_none(),
        }
    }
}
