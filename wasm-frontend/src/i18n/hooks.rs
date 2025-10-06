use yew::prelude::*;
use super::context::{I18nContext, I18nAction};
use super::core::Language;
use std::collections::HashMap;

#[hook]
pub fn use_i18n() -> UseReducerHandle<I18nContext> {
    use_context::<UseReducerHandle<I18nContext>>()
        .expect("I18nContext not found")
}

#[hook]
pub fn use_translation(key: &str) -> String {
    let ctx = use_i18n();
    ctx.i18n.t(key).to_string()
}

#[hook]
pub fn use_translation_params(key: &str, params: HashMap<String, String>) -> String {
    let ctx = use_i18n();
    ctx.i18n.t_with_params(key, &params).to_string()
}

#[hook]
pub fn use_translation_plural(key: &str, count: i64) -> String {
    let ctx = use_i18n();
    ctx.i18n.as_ref().t_plural(key, count).to_string()
}

#[hook]
pub fn use_language() -> (Language, Callback<Language>) {
    let ctx = use_i18n();
    let current_lang = ctx.language.clone(); // Access language directly from context
    
    let change_lang = {
        let ctx = ctx.clone();
        Callback::from(move |lang: Language| {
            ctx.dispatch(I18nAction::ChangeLanguage(lang));
        })
    };
    
    (current_lang, change_lang)
}

