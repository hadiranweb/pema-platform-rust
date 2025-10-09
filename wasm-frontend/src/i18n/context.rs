use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use super::core::{I18n, Language};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub i18n: Rc<RefCell<I18n>>,
    pub version: usize,
}

impl I18nContext {
    pub fn new(default_language: Language) -> Self {
        Self {
            i18n: Rc::new(RefCell::new(I18n::new(default_language))),
            version: 0,
        }
    }
    
    pub fn t(&self, key: &str) -> String {
        self.i18n.borrow().t(key)
    }
    
    pub fn t_with_params(&self, key: &str, params: &HashMap<&str, &str>) -> String {
        self.i18n.borrow().t_with_params(key, params)
    }
    
    pub fn t_plural(&self, key: &str, count: i64) -> String {
        self.i18n.borrow().t_plural(key, count)
    }
    
    pub fn current_language(&self) -> Language {
        self.i18n.borrow().current_language()
    }
}

pub enum I18nAction {
    ChangeLanguage(Language),
    LoadTranslations(Language, HashMap<String, String>),
}

impl Reducible for I18nContext {
    type Action = I18nAction;
    
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_ctx = (*self).clone();
        
        match action {
            I18nAction::ChangeLanguage(lang) => {
                new_ctx.i18n.borrow_mut().set_language(lang);
                new_ctx.version += 1;
            }
            I18nAction::LoadTranslations(lang, trans) => {
                new_ctx.i18n.borrow_mut().load_translations(lang, trans);
                new_ctx.version += 1;
            }
        }
        
        Rc::new(new_ctx)
    }
}

#[derive(Properties, PartialEq)]
pub struct I18nProviderProps {
    pub children: Children,
    #[prop_or(Language::Persian)]
    pub default_language: Language,
    #[prop_or_default]
    pub translations: Option<HashMap<Language, HashMap<String, String>>>,
}

#[function_component(I18nProvider)]
pub fn i18n_provider(props: &I18nProviderProps) -> Html {
    let ctx = use_reducer(|| {
                let ctx = I18nContext::new(props.default_language);
        
        if let Some(translations) = &props.translations {
            for (lang, trans) in translations {
                ctx.i18n.borrow_mut().load_translations(*lang, trans.clone());
            }
        }
        
        ctx
    });
    
    let lang = ctx.current_language();
    
    html! {
        <ContextProvider<UseReducerHandle<I18nContext>> context={ctx}>
            <div dir={lang.direction()} lang={lang.code()}>
                { for props.children.iter() }
            </div>
        </ContextProvider<UseReducerHandle<I18nContext>>>
    }
}

