use yew::prelude::*;
use std::rc::Rc;
use super::core::{I18n, Language};

#[derive(Clone, Debug, PartialEq)]
pub enum I18nAction {
    ChangeLanguage(Language),
}

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub i18n: Rc<I18n>,
    pub language: Language,
    pub version: u64, // Added for memoization dependency
}

impl Reducible for I18nContext {
    type Action = I18nAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            I18nAction::ChangeLanguage(new_lang) => {
                if self.language == new_lang {
                    self
                } else {
                    Rc::new(Self {
                        i18n: Rc::new(I18n::new(new_lang)),
                        language: new_lang,
                        version: self.version + 1,
                    })
                }
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct I18nProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let i18n_context = use_reducer(|| I18nContext {
        i18n: Rc::new(I18n::new(Language::English)),
        language: Language::English,
        version: 0,
    });

    html! {
        <ContextProvider<UseReducerHandle<I18nContext>> context={i18n_context}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<I18nContext>>>
    }
}

