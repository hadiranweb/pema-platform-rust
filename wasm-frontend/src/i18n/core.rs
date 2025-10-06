use std::borrow::Cow;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::rc::Rc;

pub type I18nString = Cow<'static, str>;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Farsi,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Farsi => "fa",
        }
    }
}

#[derive(PartialEq)]
pub struct I18n {
    language: Language,
    translations: HashMap<String, String>,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        let translations = Self::load_translations(language);
        I18n { language, translations }
    }

    fn load_translations(language: Language) -> HashMap<String, String> {
        let json_str = match language {
            Language::English => include_str!("en.json"),
            Language::Farsi => include_str!("fa.json"),
        };
        serde_json::from_str(json_str).unwrap_or_default()
    }

    pub fn t(&self, key: &str) -> I18nString {
        self.translations
            .get(key)
            .map(|s| Cow::Owned(s.clone()))
            .unwrap_or_else(move || Cow::Owned(key.to_string()))
    }

    pub fn t_with_params(&self, key: &str, params: &HashMap<String, String>) -> I18nString {
        let mut translated = self.t(key).to_string();
        for (param_key, param_value) in params {
            translated = translated.replace(&format!("{{{{{}}}}}", param_key), param_value);
        }
        Cow::Owned(translated)
    }

    pub fn t_plural(&self, key: &str, count: i64) -> I18nString {
        let plural_key = match self.language {
            Language::English => {
                if count == 1 { format!("{}_one", key) } else { format!("{}_other", key) }
            },
            Language::Farsi => {
                // Simplified pluralization for Farsi, usually more complex
                if count == 0 { format!("{}_zero", key) }
                else if count == 1 { format!("{}_one", key) }
                else { format!("{}_other", key) }
            }
        };
        self.t(&plural_key)
    }
}

