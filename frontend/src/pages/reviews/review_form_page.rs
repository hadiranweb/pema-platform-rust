use yew::prelude::*;
use yew_router::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::components::forms::review_form::ReviewForm;
use crate::services::review_service::{ReviewService, Review, CreateReview, UpdateReview};
use crate::router::routes::AppRoute;

#[derive(Properties, PartialEq)]
pub struct ReviewFormPageProps {
    pub id: Option<String>,
}

#[function_component(ReviewFormPage)]
pub fn review_form_page(props: &ReviewFormPageProps) -> Html {
    let review_state = use_state(|| None::<Review>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let review_id = props.id.clone();

    use_effect_with_deps(move |review_id| {
        if let Some(id) = review_id {
            let review_state = review_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match ReviewService::get_review_by_id(id.clone()).await {
                    Ok(review) => {
                        review_state.set(Some(review));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch review:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        }
        || ()
    }, review_id);

    let on_submit_form = Callback::from(move |_: ()| {
        navigator.push(&AppRoute::Reviews);
    });

    let on_cancel_form = Callback::from(move |_: ()| {
        navigator.push(&AppRoute::Reviews);
    });

    html! {
        <div class="review-form-page">
            <h1>{ if props.id.is_some() { "Edit Review" } else { "Create Review" } }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            
            { if props.id.is_some() && review_state.is_none() && !*loading_state {
                html! { <p>{ "Review not found or could not be loaded." }</p> }
            } else {
                html! {
                    <ReviewForm 
                        review={(*review_state).clone()}
                        on_submit={on_submit_form}
                        on_cancel={on_cancel_form}
                    />
                }
            } }
        </div>
    }
}

