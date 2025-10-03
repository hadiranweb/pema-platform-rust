use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "wasm_auth_backend"])]
    fn authenticate(username: &str, password: &str) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "wasm_auth_backend"])]
    fn validate_token(token: &str) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "wasm_auth_backend"])]
    fn logout() -> JsValue;
}

pub struct AuthService;

impl AuthService {
    pub async fn login(username: &str, password: &str) -> Result<String, String> {
        // Try to call the WASM auth backend if available
        if let Some(window) = web_sys::window() {
            if let Ok(auth_backend) = js_sys::Reflect::get(&window, &"wasm_auth_backend".into()) {
                if !auth_backend.is_undefined() {
                    // WASM backend is available, try to use it
                    let result = authenticate(username, password);
                    // Process the result from WASM backend
                    // For now, we'll still use demo logic
                }
            }
        }
        
        // Demo authentication logic
        if username == "admin" && password == "password" {
            Ok("demo_token_12345".to_string())
        } else {
            Err("نام کاربری یا رمز عبور اشتباه است".to_string())
        }
    }
    
    pub fn is_authenticated() -> bool {
        // Check if user has valid token in localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(token)) = storage.get_item("auth_token") {
                    return !token.is_empty();
                }
            }
        }
        false
    }
    
    pub fn get_current_user() -> Option<String> {
        // Return current user info if authenticated
        if Self::is_authenticated() {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(username)) = storage.get_item("username") {
                        return Some(username);
                    }
                }
            }
        }
        None
    }
    
    pub fn logout() -> Result<(), String> {
        // Clear authentication state from localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("auth_token");
                let _ = storage.remove_item("username");
            }
        }
        
        // Call WASM backend logout if available
        if let Some(window) = web_sys::window() {
            if let Ok(auth_backend) = js_sys::Reflect::get(&window, &"wasm_auth_backend".into()) {
                if !auth_backend.is_undefined() {
                    let _ = logout();
                }
            }
        }
        
        Ok(())
    }
}
