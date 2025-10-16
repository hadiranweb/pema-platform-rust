use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::auth_service::AuthService;
use crate::components::common::input::Input;

#[derive(Properties, PartialEq)]
pub struct PhoneAuthProps {
    pub on_success: Callback<String>,
    pub on_error: Callback<String>,
    pub mode: AuthMode,
}

#[derive(Clone, PartialEq)]
pub enum AuthMode {
    Login,
    Register,
}

pub enum PhoneAuthMsg {
    UpdatePhone(String),
    UpdatePassword(String),
    UpdateConfirmPassword(String),
    UpdateName(String),
    Submit,
    ToggleMode,
    SendOtp,
    UpdateOtp(String),
    VerifyOtp,
    SetLoading(bool),
    SetError(Option<String>),
    SetStep(AuthStep),
}

#[derive(Clone, PartialEq)]
pub enum AuthStep {
    PhonePassword,
    OtpVerification,
    Completed,
}

pub struct PhoneAuth {
    phone: String,
    password: String,
    confirm_password: String,
    name: String,
    otp: String,
    loading: bool,
    error: Option<String>,
    mode: AuthMode,
    step: AuthStep,
    auth_service: AuthService,
}

impl Component for PhoneAuth {
    type Message = PhoneAuthMsg;
    type Properties = PhoneAuthProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            phone: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            name: String::new(),
            otp: String::new(),
            loading: false,
            error: None,
            mode: ctx.props().mode.clone(),
            step: AuthStep::PhonePassword,
            auth_service: AuthService::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PhoneAuthMsg::UpdatePhone(phone) => {
                // فرمت کردن شماره تلفن
                self.phone = self.format_phone_number(phone);
                true
            }
            PhoneAuthMsg::UpdatePassword(password) => {
                self.password = password;
                true
            }
            PhoneAuthMsg::UpdateConfirmPassword(password) => {
                self.confirm_password = password;
                true
            }
            PhoneAuthMsg::UpdateName(name) => {
                self.name = name;
                true
            }
            PhoneAuthMsg::UpdateOtp(otp) => {
                // فقط اعداد مجاز
                if otp.chars().all(|c| c.is_ascii_digit()) && otp.len() <= 6 {
                    self.otp = otp;
                }
                true
            }
            PhoneAuthMsg::Submit => {
                if self.validate_form() {
                    self.submit_form(ctx);
                }
                true
            }
            PhoneAuthMsg::SendOtp => {
                self.send_otp(ctx);
                true
            }
            PhoneAuthMsg::VerifyOtp => {
                self.verify_otp(ctx);
                true
            }
            PhoneAuthMsg::ToggleMode => {
                self.mode = match self.mode {
                    AuthMode::Login => AuthMode::Register,
                    AuthMode::Register => AuthMode::Login,
                };
                self.step = AuthStep::PhonePassword;
                self.error = None;
                true
            }
            PhoneAuthMsg::SetLoading(loading) => {
                self.loading = loading;
                true
            }
            PhoneAuthMsg::SetError(error) => {
                self.error = error;
                true
            }
            PhoneAuthMsg::SetStep(step) => {
                self.step = step;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="phone-auth-container">
                <div class="auth-card">
                    <div class="auth-header">
                        <h2 class="auth-title">
                            {match (&self.mode, &self.step) {
                                (AuthMode::Login, AuthStep::PhonePassword) => "ورود با شماره تلفن",
                                (AuthMode::Register, AuthStep::PhonePassword) => "ثبت‌نام با شماره تلفن",
                                (_, AuthStep::OtpVerification) => "تأیید کد",
                                (_, AuthStep::Completed) => "تکمیل شد",
                            }}
                        </h2>
                        <p class="auth-subtitle">
                            {match (&self.mode, &self.step) {
                                (AuthMode::Login, AuthStep::PhonePassword) => "شماره تلفن و رمز عبور خود را وارد کنید",
                                (AuthMode::Register, AuthStep::PhonePassword) => "اطلاعات خود را برای ثبت‌نام وارد کنید",
                                (_, AuthStep::OtpVerification) => "کد تأیید ارسال شده به شماره تلفن خود را وارد کنید",
                                (_, AuthStep::Completed) => "احراز هویت با موفقیت انجام شد",
                            }}
                        </p>
                    </div>

                    {if let Some(error) = &self.error {
                        html! {
                            <div class="error-message">
                                <i class="icon-error"></i>
                                <span>{error}</span>
                            </div>
                        }
                    } else {
                        html! {}
                    }}

                    <form class="auth-form" onsubmit={link.callback(|e: SubmitEvent| {
                        e.prevent_default();
                        PhoneAuthMsg::Submit
                    })}>
                        {match &self.step {
                            AuthStep::PhonePassword => self.render_phone_password_form(ctx),
                            AuthStep::OtpVerification => self.render_otp_form(ctx),
                            AuthStep::Completed => self.render_completed(ctx),
                        }}
                    </form>

                    {if self.step == AuthStep::PhonePassword {
                        html! {
                            <div class="auth-footer">
                                <p class="toggle-mode">
                                    {match self.mode {
                                        AuthMode::Login => "حساب کاربری ندارید؟",
                                        AuthMode::Register => "قبلاً ثبت‌نام کرده‌اید؟",
                                    }}
                                    <button 
                                        type="button" 
                                        class="link-button"
                                        onclick={link.callback(|_| PhoneAuthMsg::ToggleMode)}
                                    >
                                        {match self.mode {
                                            AuthMode::Login => "ثبت‌نام کنید",
                                            AuthMode::Register => "وارد شوید",
                                        }}
                                    </button>
                                </p>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        }
    }
}

impl PhoneAuth {
    fn render_phone_password_form(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                {if self.mode == AuthMode::Register {
                    html! {
                        <Input
                            input_type="text"
                            name="name"
                            placeholder="نام و نام خانوادگی"
                            value={self.name.clone()}
                            onchange={link.callback(PhoneAuthMsg::UpdateName)}
                            required=true
                            icon="user"
                        />
                    }
                } else {
                    html! {}
                }}

                <div class="phone-input-container">
                    <Input
                        input_type="tel"
                        name="phone"
                        placeholder="شماره تلفن همراه"
                        value={self.phone.clone()}
                        onchange={link.callback(PhoneAuthMsg::UpdatePhone)}
                        required=true
                        icon="phone"
                        dir="ltr"
                    />
                    <div class="phone-prefix">
                        <span>{"+98"}</span>
                    </div>
                </div>

                <Input
                    input_type="password"
                    name="password"
                    placeholder="رمز عبور"
                    value={self.password.clone()}
                    onchange={link.callback(PhoneAuthMsg::UpdatePassword)}
                    required=true
                    icon="lock"
                />

                {if self.mode == AuthMode::Register {
                    html! {
                        <Input
                            input_type="password"
                            name="confirm_password"
                            placeholder="تکرار رمز عبور"
                            value={self.confirm_password.clone()}
                            onchange={link.callback(PhoneAuthMsg::UpdateConfirmPassword)}
                            required=true
                            icon="lock"
                        />
                    }
                } else {
                    html! {}
                }}

                <button 
                    type="submit" 
                    class="btn btn-primary btn-full"
                    disabled={self.loading}
                >
                    {if self.loading {
                        html! {
                            <>
                                <span class="spinner"></span>
                                {"در حال پردازش..."}
                            </>
                        }
                    } else {
                        html! {
                            match self.mode {
                                AuthMode::Login => "ورود",
                                AuthMode::Register => "ثبت‌نام",
                            }
                        }
                    }}
                </button>
            </>
        }
    }

    fn render_otp_form(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                <div class="otp-info">
                    <p>{"کد تأیید به شماره"} <strong>{&self.format_display_phone()}</strong> {"ارسال شد"}</p>
                </div>

                <div class="otp-input-container">
                    <Input
                        input_type="text"
                        name="otp"
                        placeholder="کد تأیید 6 رقمی"
                        value={self.otp.clone()}
                        onchange={link.callback(PhoneAuthMsg::UpdateOtp)}
                        required=true
                        icon="key"
                        maxlength=6
                        dir="ltr"
                    />
                </div>

                <div class="otp-actions">
                    <button 
                        type="button"
                        class="btn btn-secondary"
                        onclick={link.callback(|_| PhoneAuthMsg::SendOtp)}
                        disabled={self.loading}
                    >
                        {"ارسال مجدد کد"}
                    </button>

                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled={self.loading || self.otp.len() != 6}
                        onclick={link.callback(|_| PhoneAuthMsg::VerifyOtp)}
                    >
                        {if self.loading {
                            html! {
                                <>
                                    <span class="spinner"></span>
                                    {"تأیید..."}
                                </>
                            }
                        } else {
                            "تأیید کد"
                        }}
                    </button>
                </div>
            </>
        }
    }

    fn render_completed(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="auth-success">
                <div class="success-icon">
                    <i class="icon-check-circle"></i>
                </div>
                <h3>{"احراز هویت موفق"}</h3>
                <p>{"به پلتفرم خوش آمدید!"}</p>
            </div>
        }
    }

    fn format_phone_number(&self, input: String) -> String {
        // حذف کاراکترهای غیرعددی
        let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
        
        // حذف پیش‌شماره ایران اگر وارد شده باشد
        let clean_digits = if digits.starts_with("98") && digits.len() > 10 {
            digits[2..].to_string()
        } else if digits.starts_with("0") && digits.len() > 10 {
            digits[1..].to_string()
        } else {
            digits
        };

        // محدود کردن به 10 رقم
        if clean_digits.len() > 10 {
            clean_digits[..10].to_string()
        } else {
            clean_digits
        }
    }

    fn format_display_phone(&self) -> String {
        if self.phone.len() == 10 {
            format!("+98 {} {} {}", 
                &self.phone[..3], 
                &self.phone[3..6], 
                &self.phone[6..])
        } else {
            format!("+98 {}", self.phone)
        }
    }

    fn validate_form(&self) -> bool {
        // اعتبارسنجی شماره تلفن
        if self.phone.len() != 10 || !self.phone.starts_with('9') {
            return false;
        }

        // اعتبارسنجی رمز عبور
        if self.password.len() < 8 {
            return false;
        }

        // اعتبارسنجی تکرار رمز عبور در حالت ثبت‌نام
        if self.mode == AuthMode::Register {
            if self.name.is_empty() {
                return false;
            }
            if self.password != self.confirm_password {
                return false;
            }
        }

        true
    }

    fn submit_form(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let phone = format!("+98{}", self.phone);
        let password = self.password.clone();
        let name = self.name.clone();
        let mode = self.mode.clone();
        let auth_service = self.auth_service.clone();

        ctx.link().send_message(PhoneAuthMsg::SetLoading(true));

        spawn_local(async move {
            let result = match mode {
                AuthMode::Login => {
                    auth_service.login_with_phone(phone, password).await
                }
                AuthMode::Register => {
                    auth_service.register_with_phone(phone, password, name).await
                }
            };

            match result {
                Ok(_) => {
                    link.send_message(PhoneAuthMsg::SetStep(AuthStep::OtpVerification));
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                }
                Err(e) => {
                    link.send_message(PhoneAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                }
            }
        });
    }

    fn send_otp(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let phone = format!("+98{}", self.phone);
        let auth_service = self.auth_service.clone();

        ctx.link().send_message(PhoneAuthMsg::SetLoading(true));

        spawn_local(async move {
            match auth_service.send_sms_otp(phone).await {
                Ok(_) => {
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                }
                Err(e) => {
                    link.send_message(PhoneAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                }
            }
        });
    }

    fn verify_otp(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let phone = format!("+98{}", self.phone);
        let otp = self.otp.clone();
        let auth_service = self.auth_service.clone();
        let on_success = ctx.props().on_success.clone();

        ctx.link().send_message(PhoneAuthMsg::SetLoading(true));

        spawn_local(async move {
            match auth_service.verify_sms_otp(phone.clone(), otp).await {
                Ok(token) => {
                    link.send_message(PhoneAuthMsg::SetStep(AuthStep::Completed));
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                    on_success.emit(token);
                }
                Err(e) => {
                    link.send_message(PhoneAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(PhoneAuthMsg::SetLoading(false));
                }
            }
        });
    }
}

