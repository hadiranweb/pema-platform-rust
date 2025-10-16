use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen_futures::spawn_local;
use crate::services::auth_service::AuthService;
use crate::components::common::input::Input;

#[derive(Properties, PartialEq)]
pub struct EmailAuthProps {
    pub on_success: Callback<String>,
    pub on_error: Callback<String>,
    pub mode: AuthMode,
}

#[derive(Clone, PartialEq)]
pub enum AuthMode {
    Login,
    Register,
}

pub enum EmailAuthMsg {
    UpdateEmail(String),
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
    EmailPassword,
    OtpVerification,
    Completed,
}

pub struct EmailAuth {
    email: String,
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

impl Component for EmailAuth {
    type Message = EmailAuthMsg;
    type Properties = EmailAuthProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            name: String::new(),
            otp: String::new(),
            loading: false,
            error: None,
            mode: ctx.props().mode.clone(),
            step: AuthStep::EmailPassword,
            auth_service: AuthService::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EmailAuthMsg::UpdateEmail(email) => {
                self.email = email;
                true
            }
            EmailAuthMsg::UpdatePassword(password) => {
                self.password = password;
                true
            }
            EmailAuthMsg::UpdateConfirmPassword(password) => {
                self.confirm_password = password;
                true
            }
            EmailAuthMsg::UpdateName(name) => {
                self.name = name;
                true
            }
            EmailAuthMsg::UpdateOtp(otp) => {
                self.otp = otp;
                true
            }
            EmailAuthMsg::Submit => {
                if self.validate_form() {
                    self.submit_form(ctx);
                }
                true
            }
            EmailAuthMsg::SendOtp => {
                self.send_otp(ctx);
                true
            }
            EmailAuthMsg::VerifyOtp => {
                self.verify_otp(ctx);
                true
            }
            EmailAuthMsg::ToggleMode => {
                self.mode = match self.mode {
                    AuthMode::Login => AuthMode::Register,
                    AuthMode::Register => AuthMode::Login,
                };
                self.step = AuthStep::EmailPassword;
                self.error = None;
                true
            }
            EmailAuthMsg::SetLoading(loading) => {
                self.loading = loading;
                true
            }
            EmailAuthMsg::SetError(error) => {
                self.error = error;
                true
            }
            EmailAuthMsg::SetStep(step) => {
                self.step = step;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="email-auth-container">
                <div class="auth-card">
                    <div class="auth-header">
                        <h2 class="auth-title">
                            {match (&self.mode, &self.step) {
                                (AuthMode::Login, AuthStep::EmailPassword) => "ورود با ایمیل",
                                (AuthMode::Register, AuthStep::EmailPassword) => "ثبت‌نام با ایمیل",
                                (_, AuthStep::OtpVerification) => "تأیید کد",
                                (_, AuthStep::Completed) => "تکمیل شد",
                            }}
                        </h2>
                        <p class="auth-subtitle">
                            {match (&self.mode, &self.step) {
                                (AuthMode::Login, AuthStep::EmailPassword) => "ایمیل و رمز عبور خود را وارد کنید",
                                (AuthMode::Register, AuthStep::EmailPassword) => "اطلاعات خود را برای ثبت‌نام وارد کنید",
                                (_, AuthStep::OtpVerification) => "کد تأیید ارسال شده به ایمیل خود را وارد کنید",
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
                        EmailAuthMsg::Submit
                    })}>
                        {match &self.step {
                            AuthStep::EmailPassword => self.render_email_password_form(ctx),
                            AuthStep::OtpVerification => self.render_otp_form(ctx),
                            AuthStep::Completed => self.render_completed(ctx),
                        }}
                    </form>

                    {if self.step == AuthStep::EmailPassword {
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
                                        onclick={link.callback(|_| EmailAuthMsg::ToggleMode)}
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

impl EmailAuth {
    fn render_email_password_form(&self, ctx: &Context<Self>) -> Html {
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
                            onchange={link.callback(EmailAuthMsg::UpdateName)}
                            required=true
                            icon="user"
                        />
                    }
                } else {
                    html! {}
                }}

                <Input
                    input_type="email"
                    name="email"
                    placeholder="آدرس ایمیل"
                    value={self.email.clone()}
                    onchange={link.callback(EmailAuthMsg::UpdateEmail)}
                    required=true
                    icon="email"
                />

                <Input
                    input_type="password"
                    name="password"
                    placeholder="رمز عبور"
                    value={self.password.clone()}
                    onchange={link.callback(EmailAuthMsg::UpdatePassword)}
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
                            onchange={link.callback(EmailAuthMsg::UpdateConfirmPassword)}
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
                    <p>{"کد تأیید به آدرس"} <strong>{&self.email}</strong> {"ارسال شد"}</p>
                </div>

                <Input
                    input_type="text"
                    name="otp"
                    placeholder="کد تأیید 6 رقمی"
                    value={self.otp.clone()}
                    onchange={link.callback(EmailAuthMsg::UpdateOtp)}
                    required=true
                    icon="key"
                    maxlength=6
                />

                <div class="otp-actions">
                    <button 
                        type="button"
                        class="btn btn-secondary"
                        onclick={link.callback(|_| EmailAuthMsg::SendOtp)}
                        disabled={self.loading}
                    >
                        {"ارسال مجدد کد"}
                    </button>

                    <button 
                        type="submit" 
                        class="btn btn-primary"
                        disabled={self.loading || self.otp.len() != 6}
                        onclick={link.callback(|_| EmailAuthMsg::VerifyOtp)}
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

    fn validate_form(&self) -> bool {
        // اعتبارسنجی ایمیل
        if self.email.is_empty() || !self.email.contains('@') {
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
        let email = self.email.clone();
        let password = self.password.clone();
        let name = self.name.clone();
        let mode = self.mode.clone();
        let auth_service = self.auth_service.clone();

        ctx.link().send_message(EmailAuthMsg::SetLoading(true));

        spawn_local(async move {
            let result = match mode {
                AuthMode::Login => {
                    auth_service.login_with_email(email, password).await
                }
                AuthMode::Register => {
                    auth_service.register_with_email(email, password, name).await
                }
            };

            match result {
                Ok(_) => {
                    link.send_message(EmailAuthMsg::SetStep(AuthStep::OtpVerification));
                    link.send_message(EmailAuthMsg::SetLoading(false));
                }
                Err(e) => {
                    link.send_message(EmailAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(EmailAuthMsg::SetLoading(false));
                }
            }
        });
    }

    fn send_otp(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let email = self.email.clone();
        let auth_service = self.auth_service.clone();

        ctx.link().send_message(EmailAuthMsg::SetLoading(true));

        spawn_local(async move {
            match auth_service.send_email_otp(email).await {
                Ok(_) => {
                    link.send_message(EmailAuthMsg::SetLoading(false));
                }
                Err(e) => {
                    link.send_message(EmailAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(EmailAuthMsg::SetLoading(false));
                }
            }
        });
    }

    fn verify_otp(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let email = self.email.clone();
        let otp = self.otp.clone();
        let auth_service = self.auth_service.clone();
        let on_success = ctx.props().on_success.clone();

        ctx.link().send_message(EmailAuthMsg::SetLoading(true));

        spawn_local(async move {
            match auth_service.verify_email_otp(email.clone(), otp).await {
                Ok(token) => {
                    link.send_message(EmailAuthMsg::SetStep(AuthStep::Completed));
                    link.send_message(EmailAuthMsg::SetLoading(false));
                    on_success.emit(token);
                }
                Err(e) => {
                    link.send_message(EmailAuthMsg::SetError(Some(e.to_string())));
                    link.send_message(EmailAuthMsg::SetLoading(false));
                }
            }
        });
    }
}

