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
    let translation = use_memo(
        (key.to_string(), ctx.version),
        |(k, _)| ctx.t(k)
    );
    (*translation).clone()
}

#[hook]
pub fn use_translation_params(key: &str, params: HashMap<&str, &str>) -> String {
    let ctx = use_i18n();
    let translation = use_memo(
        (key.to_string(), params.clone(), ctx.version),
        |(k, p, _)| ctx.t_with_params(k, p)
    );
    (*translation).clone()
}

#[hook]
pub fn use_translation_plural(key: &str, count: i64) -> String {
    let ctx = use_i18n();
    let translation = use_memo(
        (key.to_string(), count, ctx.version),
        |(k, c, _)| ctx.t_plural(k, *c)
    );
    (*translation).clone()
}

#[hook]
pub fn use_language() -> (Language, Callback<Language>) {
    let ctx = use_i18n();
    let current_lang = ctx.current_language();
    
    let change_lang = {
        let ctx = ctx.clone();
        Callback::from(move |lang: Language| {
            ctx.dispatch(I18nAction::ChangeLanguage(lang));
        })
    };
    
    (current_lang, change_lang)
}

