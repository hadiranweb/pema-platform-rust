mod i18n;

use yew::prelude::*;
use std::collections::HashMap;
use i18n::{I18nProvider, Language, use_translation, use_translation_plural, use_translation_params, use_language, format_number_persian, format_date_jalali, LanguageSwitcher, Trans};
use serde_json;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // Prepare translations
    let translations = load_translations_from_json();
    
    html! {
        <I18nProvider 
            default_language={Language::Persian}
            translations={Some(translations)}
        >
            <MainApp />
        </I18nProvider>
    }
}

#[function_component(MainApp)]
fn main_app() -> Html {
    html! {
        <div class="app-container">
            <Header />
            <MainContent />
            <Footer />
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    let welcome = use_translation("app.welcome");
    
    html! {
        <header class="app-header">
            <h1>{ welcome }</h1>
            <LanguageSwitcher show_flags={true} />
        </header>
    }

#[function_component(MainContent)]
fn main_content() -> Html {
    let count = use_state(|| 5);
    let message = use_translation_plural("messages.count", *count);
    
    let mut params = HashMap::new();
    params.insert("name", "علی");
    let greeting = use_translation_params("user.greeting", params);
    
    let increment = {
        let count = count.clone();
        Callback::from(move |_| count.set(*count + 1))
    };
    
    let decrement = {
        let count = count.clone();
        Callback::from(move |_| {
            if *count > 0 {
                count.set(*count - 1);
            }
        })
    };
    
    html! {
        <main class="main-content">
            <section class="greeting-section">
                <p>{ greeting }</p>
            </section>
            
            <section class="counter-section">
                <h2>{ use_translation("counter.title") }</h2>
                <div class="counter-display">
                    <p>{ message }</p>
                </div>
                <div class="counter-controls">
                    <button onclick={decrement}>{ "-" }</button>
                    <span>{ format_number_persian(*count) }</span>
                    <button onclick={increment}>{ "+" }</button>
                </div>
            </section>
            
            <section class="trans-example">
                <Trans 
                    key="complex.message"
                    params={
                        let mut map = HashMap::new();
                        map.insert("app".to_string(), "Yew".to_string());
                        map
                    }
                />
            </section>
            
            <FormExample />
        </main>
    }
}

#[function_component(FormExample)]
fn form_example() -> Html {
    let name = use_state(|| String::new());
    let email = use_state(|| String::new());
    
    let onsubmit = {
        let name = name.clone();
        let email = email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            web_sys::console::log_1(
                &format!("Name: {}, Email: {}", *name, *email).into()
            );
        })
    };
    
    html! {
        <section class="form-section">
                <h2>{ use_translation("form.title") }</h2>
            <form {onsubmit}>
                <div class="form-group">
                    <label>{ use_translation("form.name") }</label>
                    <input 
                        type="text"
                        value={(*name).clone()}
                        oninput={
                            let name = name.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = 
                                    e.target_unchecked_into();
                                name.set(input.value());
                            })
                        }
                    />
                </div>
                
                <div class="form-group">
                    <label>{ use_translation("form.email") }</label>
                    <input 
                        type="email"
                        value={(*email).clone()}
                        oninput={
                            let email = email.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = 
                                    e.target_unchecked_into();
                                email.set(input.value());
                            })
                        }
                    />
                </div>
                
                <button type="submit">
                    { use_translation("form.submit") }
                </button>
            </form>
        </section>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    let (lang, _) = use_language();
    let date_text = format_date_jalali(1403, 7, 13);
    
    html! {
        <footer class="app-footer">
            <p>
                { use_translation("footer.copyright") }
                {" "}
                { date_text }
            </p>
            <p>
                { format!("{}: {}", 
                    use_translation("footer.language"), 
                    lang.name()
                )}
            </p>
        </footer>
    }
}

// Helper function to load from JSON (compile-time embedding)
fn load_translations_from_json() -> HashMap<Language, HashMap<String, String>> {
    let mut translations = HashMap::new();
    
    // Embed JSON files at compile time
    let fa_json = include_str!("../locales/fa.json");
    let en_json = include_str!("../locales/en.json");
    
    // Parse and flatten
    if let Ok(fa_map) = parse_and_flatten_json(fa_json) {
        translations.insert(Language::Persian, fa_map);
    }
    
    if let Ok(en_map) = parse_and_flatten_json(en_json) {
        translations.insert(Language::English, en_map);
    }
    
    translations
}

// Flatten nested JSON to dot notation
fn parse_and_flatten_json(json: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let mut result = HashMap::new();
    flatten_json_value(&value, String::new(), &mut result);
    Ok(result)
}

fn flatten_json_value(
    value: &serde_json::Value,
    prefix: String,
    result: &mut HashMap<String, String>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_json_value(val, new_prefix, result);
            }
        }
        serde_json::Value::String(s) => {
            result.insert(prefix, s.clone());
        }
        _ => {}
    }
}


}
