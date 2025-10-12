use yew::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::components::forms::page_form::PageForm;
use crate::services::admin_service::{AdminService, Page, CreatePage, AdminPageUpdateDto};

#[function_component(AdminPagesPage)]
pub fn admin_pages_page() -> Html {
    let pages_state = use_state(|| None::<Vec<Page>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);
    let show_add_form = use_state(|| false);
    let editing_page = use_state(|| None::<Page>);

    let fetch_pages = { 
        let pages_state = pages_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let pages_state = pages_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match AdminService::get_all_pages().await {
                    Ok(pages) => {
                        pages_state.set(Some(pages));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch pages:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_pages| {
        fetch_pages.emit(());
        || ()
    }, fetch_pages.clone());

    let on_add_page_click = { 
        let show_add_form = show_add_form.clone();
        let editing_page = editing_page.clone();
        Callback::from(move |_| {
            show_add_form.set(true);
            editing_page.set(None);
        })
    };

    let on_edit_page_click = { 
        let show_add_form = show_add_form.clone();
        let editing_page = editing_page.clone();
        Callback::from(move |page: Page| {
            show_add_form.set(true);
            editing_page.set(Some(page));
        })
    };

    let on_delete_page_click = { 
        let fetch_pages = fetch_pages.clone();
        Callback::from(move |page_id: String| {
            let fetch_pages = fetch_pages.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match AdminService::delete_page(page_id).await {
                    Ok(_) => {
                        log!("Page deleted successfully");
                        fetch_pages.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete page:", e.to_string());
                    }
                }
            });
        })
    };

    let on_form_submit = { 
        let show_add_form = show_add_form.clone();
        let fetch_pages = fetch_pages.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
            fetch_pages.emit(());
        })
    };

    let on_form_cancel = { 
        let show_add_form = show_add_form.clone();
        Callback::from(move |_| {
            show_add_form.set(false);
        })
    };

    html! {
        <div class="admin-pages-page">
            <h1>{ "Manage Pages" }</h1>
            <button onclick={on_add_page_click}>{ "Add New Page" }</button>

            { if *show_add_form {
                html! {
                    <PageForm 
                        page={(*editing_page).clone()}
                        on_submit={on_form_submit.clone()}
                        on_cancel={on_form_cancel.clone()}
                    />
                }
            } else { html! {} } }

            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            
            <div class="page-list">
                { 
                    if let Some(pages) = &*pages_state {
                        if pages.is_empty() {
                            html! { <p>{ "No pages found." }</p> }
                        } else {
                            pages.iter().map(|page| {
                                let page_id = page.id.to_string();
                                let page_clone = page.clone();
                                let on_edit = on_edit_page_click.clone();
                                let on_delete = on_delete_page_click.clone();
                                html! {
                                    <div key={page_id.clone()} class="page-card">
                                        <h3>{ &page.title }</h3>
                                        <p>{ &page.slug }</p>
                                        <p>{ format!("Published: {}", page.is_published) }</p>
                                        <button onclick={move |_| on_edit.emit(page_clone.clone())}>{ "Edit" }</button>
                                        <button onclick={move |_| on_delete.emit(page_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading pages..." }</p> }
                    }
                }
            </div>
        </div>
    }
}

