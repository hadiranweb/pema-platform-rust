use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Persian,
    English,
}

impl Language {
    pub fn code(&self) -> &\'static str {
        match self {
            Language::Persian => "fa",
            Language::English => "en",
        }
    }
    
    pub fn name(&self) -> &\'static str {
        match self {
            Language::Persian => "فارسی",
            Language::English => "English",
        }
    }
    
    pub fn is_rtl(&self) -> bool {
        matches!(self, Language::Persian)
    }
    
    pub fn direction(&self) -> &\'static str {
        if self.is_rtl() { "rtl" } else { "ltr" }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Persian
    }
}

pub struct I18n {
    current_language: Language,
    fallback_language: Language,
    translations: HashMap<Language, HashMap<String, String>>,
}

impl I18n {
    pub fn new(default_language: Language) -> Self {
        Self {
            current_language: default_language,
            fallback_language: Language::English,
            translations: HashMap::new(),
        }
    }
    
    pub fn load_translations(&mut self, lang: Language, trans: HashMap<String, String>) {
        self.translations.insert(lang, trans);
    }
    
    pub fn load_from_json(&mut self, lang: Language, json: &str) -> Result<(), String> {
        let trans: HashMap<String, String> = serde_json::from_str(json)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        self.load_translations(lang, trans);
        Ok(())
    }
    
    pub fn t(&self, key: &str) -> String {
        if let Some(translations) = self.translations.get(&self.current_language) {
            if let Some(text) = translations.get(key) {
                return text.clone();
            }
        }
        
        if self.current_language != self.fallback_language {
            if let Some(translations) = self.translations.get(&self.fallback_language) {
                if let Some(text) = translations.get(key) {
                    return text.clone();
                }
            }
        }
        
        key.to_string()
    }
    
    pub fn t_with_params(&self, key: &str, params: &HashMap<&str, &str>) -> String {
        let mut text = self.t(key);
        for (k, v) in params {
            let placeholder = format!("{{}}", k);
            text = text.replace(&placeholder, v);
        }
        text
    }
    
    pub fn t_plural(&self, key: &str, count: i64) -> String {
        let plural_key = match self.current_language {
            Language::Persian => {
                if count == 0 {
                    format!("{}.zero", key)
                } else if count == 1 {
                    format!("{}.one", key)
                } else {
                    format!("{}.other", key)
                }
            }
            Language::English => {
                if count == 1 {
                    format!("{}.one", key)
                } else {
                    format!("{}.other", key)
                }
            }
        };
        
        let count_str = count.to_string();
        let mut params = HashMap::new();
        params.insert("count", count_str.as_str());
        self.t_with_params(&plural_key, &params)
    }
    
    pub fn set_language(&mut self, lang: Language) {
        self.current_language = lang;
    }
    
    pub fn current_language(&self) -> Language {
        self.current_language
    }
    
    pub fn set_fallback_language(&mut self, lang: Language) {
        self.fallback_language = lang;
    }
}

