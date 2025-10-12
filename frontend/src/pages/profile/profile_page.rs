use yew::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    html! {
        <div>
            <h1>{ "User Profile" }</h1>
            <p>{ "User profile details will be displayed here." }</p>
        </div>
    }
}

