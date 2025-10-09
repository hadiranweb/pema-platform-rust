use yew::prelude::*;
use super::hooks::{use_translation, use_translation_params};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct TransProps {
    pub key: AttrValue,
    #[prop_or_default]
    pub params: HashMap<String, String>,
}

#[function_component(Trans)]
pub fn trans(props: &TransProps) -> Html {
    let key_string = props.key.to_string();
    
    // Call hooks unconditionally at the top level
    let translation_without_params = use_translation(&key_string);
    let translation_with_params = use_translation_params(key_string.clone(), props.params.clone());

    let displayed_text = if props.params.is_empty() {
        translation_without_params
    } else {
        translation_with_params
    };
    
    html! {
        <span class="i18n-trans">{ displayed_text }</span>
    }
}

