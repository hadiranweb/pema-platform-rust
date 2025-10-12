use yew::prelude::*;

#[function_component(EmailAuth)]
pub fn email_auth() -> Html {
    html! {
        <div>
            <h2>{ "Email Authentication" }</h2>
            <p>{ "Email login/registration form." }</p>
        </div>
    }
}

