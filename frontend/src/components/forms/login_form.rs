use yew::prelude::*;
use yew_hooks::use_state;
use crate::components::common::Input;
use crate::components::common::Button;

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |value| email.set(value))
    };

    let on_password_change = { 
        let password = password.clone();
        Callback::from(move |value| password.set(value))
    };

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        // Handle login logic here
        log::info!("Login attempt with email: {} and password: {}", email.as_str(), password.as_str());
    });

    html! {
        <form onsubmit={on_submit}>
            <Input
                label="Email"
                input_type="email"
                value={email.to_string()}
                onchange={on_email_change}
                placeholder="Enter your email"
                required={true}
            />
            <Input
                label="Password"
                input_type="password"
                value={password.to_string()}
                onchange={on_password_change}
                placeholder="Enter your password"
                required={true}
            />
            <Button class="btn-primary">{ "Login" }</Button>
        </form>
    }
}

