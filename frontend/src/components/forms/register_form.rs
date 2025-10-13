use yew::prelude::*;
use yew_hooks::use_state;
use crate::components::common::Input;
use crate::components::common::Button;

#[function_component(RegisterForm)]
pub fn register_form() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |value| email.set(value))
    };

    let on_password_change = { 
        let password = password.clone();
        Callback::from(move |value| password.set(value))
    };

    let on_confirm_password_change = { 
        let confirm_password = confirm_password.clone();
        Callback::from(move |value| confirm_password.set(value))
    };

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        // Handle registration logic here
        log::info!("Register attempt with email: {}, password: {}, confirm_password: {}", email.as_str(), password.as_str(), confirm_password.as_str());
    });

    html! {
        <form onsubmit={on_submit}>
            <Input
                label="Email"
                input_input_type="email"
                value={email.to_string()}
                onchange={on_email_change}
                placeholder="Enter your email"
                required={true}
            />
            <Input
                label="Password"
                input_input_type="password"
                value={password.to_string()}
                onchange={on_password_change}
                placeholder="Enter your password"
                required={true}
            />
            <Input
                label="Confirm Password"
                input_input_type="password"
                value={confirm_password.to_string()}
                onchange={on_confirm_password_change}
                placeholder="Confirm your password"
                required={true}
            />
            <Button class="btn-primary">{ "Register" }</Button>
        </form>
    }
}

