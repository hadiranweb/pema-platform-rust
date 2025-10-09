use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="container text-center py-10">
            <h1 class="text-4xl font-bold mb-4">{"404 - Page Not Found"}</h1>
            <p class="text-lg">{"The page you are looking for does not exist."}</p>
            <a href="/" class="text-blue-500 hover:underline mt-4 inline-block">{"Go to Home"}</a>
        </div>
    }
}
