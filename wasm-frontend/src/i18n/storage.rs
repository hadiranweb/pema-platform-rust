#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::window;
use super::core::Language;

#[cfg(target_arch = "wasm32")]
pub fn save_language_preference(lang: Language) -> Result<(), JsValue> {
    let storage = window()
        .ok_or("No window")?
        .local_storage()?
        .ok_or("No localStorage")?;
    storage.set_item("app_language", lang.code())?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn load_language_preference() -> Option<Language> {
    let storage = window()?.local_storage().ok()??;
    let lang_code = storage.get_item("app_language").ok()??;
    match lang_code.as_str() {
        "fa" => Some(Language::Persian),
        "en" => Some(Language::English),
        _ => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_language_preference(_lang: Language) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_language_preference() -> Option<Language> {
    None
}

