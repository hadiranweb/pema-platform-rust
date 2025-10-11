use yew::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use crate::components::Card;
use crate::models::wallet::{CreateWalletRequest, Wallet};

// Import the WASM function from wasm-general-backend
#[wasm_bindgen(module = "/pkg/wasm_general_backend.js")]
extern "C" {
    #[wasm_bindgen(js_name = createNewWallet)]
    fn create_new_wallet_wasm(user_id: String, currency: String, initial_balance: Option<f64>) -> Result<JsValue, JsValue>;
}

#[function_component(Profile)]
pub fn profile() -> Html {
    let currency_state = use_state(|| "USD".to_string());
    let initial_balance_state = use_state(|| None);
    let message_state = use_state(|| "".to_string());

    let on_currency_change = Callback::from({
        let currency_state = currency_state.clone();
        move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            currency_state.set(input.value());
        }
    });

    let on_initial_balance_change = Callback::from({
        let initial_balance_state = initial_balance_state.clone();
        move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value().parse::<f64>().ok();
            initial_balance_state.set(value);
        }
    });

    let on_submit = Callback::from({
        let currency_state = currency_state.clone();
        let initial_balance_state = initial_balance_state.clone();
        let message_state = message_state.clone();

        move |e: SubmitEvent| {
            e.prevent_default();
            let currency = (*currency_state).clone();
            let initial_balance = (*initial_balance_state).clone();
            // TODO: Get actual user_id from authentication context
            let user_id = "a1b2c3d4-e5f6-7890-1234-567890abcdef".to_string(); // Placeholder user_id

            let message_state = message_state.clone();
            spawn_local(async move {
                match create_new_wallet_wasm(user_id, currency, initial_balance).await {
                    Ok(wallet_js_value) => {
                        let wallet: Wallet = serde_wasm_bindgen::from_value(wallet_js_value)
                            .expect("Failed to deserialize wallet");
                        message_state.set(format!("Wallet created successfully: {:?}", wallet));
                    }
                    Err(e) => {
                        message_state.set(format!("Failed to create wallet: {:?}", e.as_string()));
                    }
                }
            });
        }
    });

    html! {
        <div class="profile-page">
            <div class="profile-header">
                <h1>{"پروفایل کاربری"}</h1>
                <p>{"مدیریت اطلاعات شخصی و تنظیمات حساب"}</p>
            </div>

            <div class="profile-details">
                <Card title="اطلاعات شخصی" class="profile-card">
                    <div class="detail-item">
                        <span class="detail-label">{"نام و نام خانوادگی:"}</span>
                        <span class="detail-value">{"جان دو"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"ایمیل:"}</span>
                        <span class="detail-value">{"john.doe@example.com"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"شماره تماس:"}</span>
                        <span class="detail-value">{"۰۹۱۲۳۴۵۶۷۸۹"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"نقش:"}</span>
                        <span class="detail-value">{"مدیر سیستم"}</span>
                    </div>
                </Card>

                <Card title="تنظیمات حساب" class="profile-card">
                    <div class="setting-item">
                        <span class="setting-label">{"تغییر رمز عبور"}</span>
                        <button class="button secondary small">{"تغییر"}</button>
                    </div>
                    <div class="setting-item">
                        <span class="setting-label">{"فعال‌سازی احراز هویت دو مرحله‌ای"}</span>
                        <button class="button secondary small">{"فعال‌سازی"}</button>
                    </div>
                    <div class="setting-item">
                        <span class="setting-label">{"مدیریت اعلان‌ها"}</span>
                        <button class="button secondary small">{"تنظیمات"}</button>
                    </div>
                </Card>

                <Card title="ایجاد کیف پول جدید" class="wallet-creation-card">
                    <form onsubmit={on_submit}>
                        <div class="form-group">
                            <label for="currency">{"واحد پول:"}</label>
                            <input
                                id="currency"
                                type="text"
                                value={(*currency_state).clone()}
                                onchange={on_currency_change}
                                placeholder="مثال: USD, IRR"
                            />
                        </div>
                        <div class="form-group">
                            <label for="initial_balance">{"موجودی اولیه (اختیاری):"}</label>
                            <input
                                id="initial_balance"
                                type="number"
                                value={initial_balance_state.map_or("".to_string(), |b| b.to_string())}
                                onchange={on_initial_balance_change}
                                placeholder="مثال: 100.00"
                            />
                        </div>
                        <button type="submit" class="button primary">{"ایجاد کیف پول"}</button>
                    </form>
                    { if !message_state.is_empty() { html! { <p class="message">{ (*message_state).clone() }</p> } } else { html! {} } }
                </Card>
            </div>
        </div>
    }
}
