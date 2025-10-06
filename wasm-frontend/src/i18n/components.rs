use yew::prelude::*;
use super::hooks::{use_language, use_i18n};
use super::core::{Language, TranslatableString};
use std::collections::HashMap;


/// Language switcher component
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
        (Language::Farsi, "🇮🇷", "فارسی"),
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
                        class={classes!(
                            "lang-btn",
                            is_active.then(|| "active")
                        )}
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

/// Trans component for complex translations with HTML
#[derive(Properties, PartialEq)]
pub struct TransProps {
        pub text_key: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub params: HashMap<String, String>,
}

#[function_component(Trans)]
pub fn trans(props: &TransProps) -> Html {
    let i18n = use_i18n();
        let translation = i18n.i18n.t_with_params(&props.text_key, &props.params);

    html! {
        <span class="i18n-trans">{ translation }</span>
    }
}

