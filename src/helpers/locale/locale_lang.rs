use fluent_templates::{LanguageIdentifier, Loader};
use locale_config::Locale;

use crate::locale::LOCALES;

// TODO move global locale stuff in its own thing
fn get_fallback_lang() -> LanguageIdentifier {
    "en-US".parse::<LanguageIdentifier>().unwrap()
}

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
            None => Locale::current()
                .to_string()
                .parse()
                .unwrap_or_else(|_| get_fallback_lang()),
        }
    }

    pub fn get_str_from_id(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.get_lang(), text_id)
    }
}
