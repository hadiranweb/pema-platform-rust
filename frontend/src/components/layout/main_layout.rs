use yew::prelude::*;
use crate::components::layout::{Header, Navbar, Footer, StarrySky};
use yewdux::prelude::use_reducer_globally;
use crate::state::auth::AuthStore;

#[derive(Properties, PartialEq)]
pub struct MainLayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(MainLayout)]
pub fn main_layout(props: &MainLayoutProps) -> Html {
    html! {
        <div class="app-container">
            <StarrySky />
            <Header />
            <Navbar />
            <main class="app-content">
                { props.children.clone() }
            </main>
            <Footer />
        </div>
    }
}

