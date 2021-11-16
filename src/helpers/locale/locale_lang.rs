use fluent_templates::LanguageIdentifier;
use locale_config::Locale;

// TODO move global locale stuff in its own thing
fn get_fallback_lang() -> LanguageIdentifier {
    "en-US".parse::<LanguageIdentifier>().unwrap()
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct LocaleLang {
    #[cfg_attr(feature = "persistence", serde(skip))]
    lang: Option<LanguageIdentifier>,
    // only used for serialization. makes sure it syncs with lang
    lang_str: Option<String>,
}

impl LocaleLang {
    pub fn new(lang: Option<LanguageIdentifier>) -> Self {
        let lang_str = match &lang {
            Some(some) => Some(some.to_string()),
            None => None,
        };

        Self { lang, lang_str }
    }

    pub fn get_lang(&mut self) -> LanguageIdentifier {
        // deserialization check
        if self.lang_str.is_some() && self.lang.is_none() {
            // got checked, lang_str is some
            let lang = match self
                .lang_str
                .to_owned()
                .unwrap()
                .parse::<LanguageIdentifier>()
            {
                Ok(lang) => lang,
                Err(_) => get_fallback_lang(),
            };

            self.lang = Some(lang);
        }

        match &self.lang {
            Some(lang) => lang.to_owned(),
            // shouldn't error
            None => Locale::current()
                .to_string()
                .parse()
                .unwrap_or_else(|_| get_fallback_lang()),
        }
    }
}
