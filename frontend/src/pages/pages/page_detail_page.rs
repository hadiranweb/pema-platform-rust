use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::components::common::spinner::Spinner;
use crate::services::api_client::fetch_page_by_id;
use crate::models::models::page::Page;

#[derive(Properties, PartialEq)]
pub struct PageDetailProps {
    pub id: i32,
}

#[function_component(PageDetailPage)]
pub fn page_detail_page(props: &PageDetailProps) -> Html {
    let page = use_state(|| None);
    let error = use_state(|| None);
    let loading = use_state(|| true);

    { // Fetch page details on component mount or when ID changes
        let page = page.clone();
        let error = error.clone();
        let loading = loading.clone();
        let page_id = props.id;
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_page_by_id(page_id).await {
                    Ok(fetched_page) => {
                        page.set(Some(fetched_page));
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
    } else if let Some(page_data) = &*page {
        html! {
            <div class="container mx-auto p-4">
                <h1 class="text-3xl font-bold mb-4">{&page_data.title}</h1>
                <p class="text-gray-600 mb-2">{format!("Slug: {}", &page_data.slug)}</p>
                <p class="text-gray-700 mb-4">{format!("Published: {}", page_data.is_published)}</p>
                <div class="prose lg:prose-xl" dangerously_set_inner_html={page_data.content.clone()} />
                <Link<Route> to={Route::PagesList}>
                    <button class="mt-8 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
                        {"Back to Pages"}
                    </button>
                </Link<Route>>
            </div>
        }
    } else {
        html! {
            <div class="text-center">{"Page not found."}
                <Link<Route> to={Route::PagesList}>
                    <button class="mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
                        {"Back to Pages"}
                    </button>
                </Link<Route>>
            </div>
        }
    }
}

