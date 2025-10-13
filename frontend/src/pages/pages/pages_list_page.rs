use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::common::spinner::Spinner;
use crate::services::api_client::fetch_pages;
use crate::models::models::page::Page;

#[function_component(PagesListPage)]
pub fn pages_list_page() -> Html {
    let pages = use_state(|| None::<Vec<models::page::Page>>);
    let error = use_state(|| None);
    let loading = use_state(|| true);

    { // Fetch pages on component mount
        let pages = pages.clone();
        let error = error.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_pages().await {
                    Ok(fetched_pages) => {
                        pages.set(Some(fetched_pages));
                    }
                    Err(err) => {
                        error.set(Some(err.to_string()));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        html! {
            <div class="flex justify-center items-center h-screen">
                <Spinner />
            </div>
        }
    } else if let Some(err) = &*error {
        html! {
            <div class="text-red-500 text-center">{ format!("Error: {}", err) }</div>
        }
    } else if let Some(pages_data) = &*pages {
        html! {
            <div class="container mx-auto p-4">
                <h1 class="text-2xl font-bold mb-4">{"All Pages"}</h1>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    { for pages_data.iter().map(|page| html! {
                        <div class="bg-white shadow-md rounded-lg p-4">
                            <h2 class="text-xl font-semibold mb-2">{&page.title}</h2>
                            <p class="text-gray-600 mb-2">{format!("Slug: {}", &page.slug)}</p>
                            <p class="text-gray-700">{format!("Published: {}", page.is_published)}</p>
                            <Link<Route> to={Route::PageDetail { id: page.id }}>
                                <button class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
                                    {"View Details"}
                                </button>
                            </Link<Route>>
                        </div>
                    }) }
                </div>
            </div>
        }
    } else {
        html! {
            <div class="text-center">{"No pages found."}
                <Link<Route> to={Route::AdminPages}>
                    <button class="mt-4 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600">
                        {"Go to Admin Pages"}
                    </button>
                </Link<Route>>
            </div>
        }
    }
}

