use yew::prelude::*;

#[function_component(PhoneAuth)]
pub fn phone_auth() -> Html {
    html! {
        <div>
            <h2>{ "Phone Authentication" }</h2>
            <p>{ "Phone number login/registration form." }</p>
        </div>
    }
}

