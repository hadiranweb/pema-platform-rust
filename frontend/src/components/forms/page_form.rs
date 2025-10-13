
use yew::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::components::common::spinner::Spinner;
use crate::services::admin_service::{AdminService, CreatePage, AdminPageUpdateDto, Page};

#[derive(Properties, PartialEq)]
pub struct PageFormProps {
    pub page: Option<Page>,
    pub on_submit: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(PageForm)]
pub fn page_form(props: &PageFormProps) -> Html {
    let title_state = use_state(|| props.page.as_ref().map_or(String::new(), |p| p.title.clone()));
    let content_state = use_state(|| props.page.as_ref().map_or(String::new(), |p| p.content.clone()));
    let slug_state = use_state(|| props.page.as_ref().map_or(String::new(), |p| p.slug.clone()));
    let is_published_state = use_state(|| props.page.as_ref().map_or(false, |p| p.is_published));
    let loading_state = use_state(|| false);
    let error_message_state = use_state(|| Option::<String>::None);

    let on_title_change = Callback::from(move |value: String| {
        
        title_state.set(value);
    });

    let on_content_change = Callback::from(move |value: String| {
        
        content_state.set(value);
    });

    let on_slug_change = Callback::from(move |value: String| {
        
        slug_state.set(value);
    });

    let on_is_published_change = Callback::from(move |value: String| {
        
        is_published_state.set(input.checked());
    });

    let on_submit = { 
        let title_state = title_state.clone();
        let content_state = content_state.clone();
        let slug_state = slug_state.clone();
        let is_published_state = is_published_state.clone();
        let loading_state = loading_state.clone();
        let error_message_state = error_message_state.clone();
        let on_submit_callback = props.on_submit.clone();
        let page = props.page.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let title = (*title_state).clone();
            let content = (*content_state).clone();
            let slug = (*slug_state).clone();
            let is_published = *is_published_state;
            let loading_state = loading_state.clone();
            let error_message_state = error_message_state.clone();
            let on_submit_callback = on_submit_callback.clone();
            let page = page.clone();

            loading_state.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let result = if let Some(existing_page) = page {
                    let update_dto = AdminPageUpdateDto {
                        title: Some(title),
                        content: Some(content),
                        slug: Some(slug),
                        is_published: Some(is_published),
                    };
                    AdminService::update_page(existing_page.id.to_string(), update_dto).await
                } else {
                    let create_page = CreatePage {
                        title,
                        content,
                        slug,
                        is_published,
                    };
                    AdminService::create_page(create_page).await
                };

                match result {
                    Ok(_) => {
                        log!("Page saved successfully");
                        error_message_state.set(None);
                        on_submit_callback.emit(());
                    },
                    Err(e) => {
                        log!("Failed to save page:", e.to_string());
                        error_message_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    let on_cancel = props.on_cancel.clone();

    html! {
        <div class="page-form">
            <h2>{ if props.page.is_some() { "Edit Page" } else { "Add New Page" } }</h2>
            <form onsubmit={on_submit}>
                <Input
                    label="Title"
                    id="page-title"
                    input_type="text"
                    value={(*title_state).clone()}
                    on_change={on_title_change}
                    placeholder="Page Title"
                />
                <Input
                    label="Content"
                    id="page-content"
                    input_type="text"
                    value={(*content_state).clone()}
                    on_change={on_content_change}
                    placeholder="Page Content"
                />
                <Input
                    label="Slug"
                    id="page-slug"
                    input_type="text"
                    value={(*slug_state).clone()}
                    on_change={on_slug_change}
                    placeholder="Page Slug"
                />
                <label for="is-published">
                    <input
                        input_type="checkbox"
                        id="is-published"
                        checked={*is_published_state}
                        onchange={on_is_published_change}
                    />
                    { "Published" }
                </label>
                { if *loading_state { html! { <Spinner /> } } else { html! {} } }
                { if let Some(msg) = &*error_message_state { html! { <p class="error-message">{ msg }</p> } } else { html! {} } }
                <Button label={ if props.page.is_some() { "Save Changes" } else { "Add Page" } } button_input_type="submit" />
                <Button label="Cancel" button_input_type="button" onclick={on_cancel} />
            </form>
        </div>
    }
}

