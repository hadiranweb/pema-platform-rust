use yew::prelude::*;
use yewdux::prelude::use_selector;
use crate::state::auth::AuthStore;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    let auth_state = use_selector(|state: &AuthStore| state.clone());

    html! {
        <div class="dashboard-page">
            <h1>{ "User Dashboard" }</h1>
            { if let Some(username) = &auth_state.username {
                html! { <p>{ format!("Welcome, {}!", username) }</p> }
            } else {
                html! { <p>{ "Welcome!" }</p> }
            } }
            { if let Some(email) = &auth_state.email {
                html! { <p>{ format!("Your email: {}", email) }</p> }
            } else {
                html! {} 
            } }
            { if let Some(user_id) = &auth_state.user_id {
                html! { <p>{ format!("Your User ID: {}", user_id) }</p> }
            } else {
                html! {} 
            } }
        </div>
    }
}

