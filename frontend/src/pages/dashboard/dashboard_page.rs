use yew::prelude::*;

#[function_component(DashboardPage)]
pub fn dashboard_page() -> Html {
    html! {
        <div>
            <h1>{ "User Dashboard" }</h1>
            <p>{ "Welcome to your personal dashboard." }</p>
        </div>
    }
}

