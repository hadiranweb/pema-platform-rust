use yew::prelude::*;
use super::hooks::{use_language, use_translation, use_translation_params};
use super::core::Language;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct LanguageSwitcherProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(true)]
    pub show_flags: bool,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &LanguageSwitcherProps) -> Html {
    let (current_lang, change_lang) = use_language();
    
    let languages = vec![
        (Language::Persian, "🇮🇷", "فارسی"),
        (Language::English, "🇬🇧", "English"),
    ];
    
    html! {
        <div class={classes!("language-switcher", props.class.clone())}>
            { for languages.iter().map(|(lang, flag, name)| {
                let is_active = lang == &current_lang;
                let lang = *lang;
                let on_click = {
                    let change_lang = change_lang.clone();
                    Callback::from(move |_| change_lang.emit(lang))
                };
                
                html! {
                    <button
                        class={classes!("lang-btn", is_active.then(|| "active"))}
                        onclick={on_click}
                    >
                        { if props.show_flags {
                            html! { <span class="flag">{flag}</span> }
                        } else {
                            html! {}
                        }}
                        <span class="name">{name}</span>
                    </button>
                }
            })}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TransProps {
    pub key: AttrValue,
    #[prop_or_default]
    pub params: HashMap<String, String>,
}

#[function_component(Trans)]
pub fn trans(props: &TransProps) -> Html {
    let key_string = props.key.to_string();
    
    let translation_output = if props.params.is_empty() {
        use_translation(key_string.clone())
    } else {
        let params_owned: HashMap<String, String> = props.params.clone();
        use_translation_params(key_string.clone(), params_owned)
    };
    
    let translation = translation_output.to_string();

    html! {
        <span class="i18n-trans">{ translation }</span>
    }
}

