use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[derive(Properties, PartialEq)]
pub struct ProtectedRouteProps {
    #[prop_or_default]
    pub children: Children,
    pub authorized: bool,
}

#[function_component(ProtectedRoute)]
pub fn protected_route(props: &ProtectedRouteProps) -> Html {
    if props.authorized {
        html! {
            { props.children.clone() }
        }
    } else {
        html! {
            <Redirect<Route> to={Route::Login} />
        }
    }
}

