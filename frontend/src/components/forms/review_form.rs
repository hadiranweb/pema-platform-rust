use yew::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::components::common::spinner::Spinner;
use crate::services::review_service::{ReviewService, Review, CreateReview, UpdateReview};
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct ReviewFormProps {
    pub review: Option<Review>,
    pub on_submit: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(ReviewForm)]
pub fn review_form(props: &ReviewFormProps) -> Html {
    let product_id_state = use_state(|| props.review.as_ref().map_or(String::new(), |r| r.product_id.to_string()));
    let rating_state = use_state(|| props.review.as_ref().map_or(String::new(), |r| r.rating.to_string()));
    let comment_state = use_state(|| props.review.as_ref().map_or(String::new(), |r| r.comment.clone().unwrap_or_default()));
    let loading_state = use_state(|| false);
    let error_message_state = use_state(|| Option::<String>::None);

    let on_product_id_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        product_id_state.set(input.value());
    });

    let on_rating_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        rating_state.set(input.value());
    });

    let on_comment_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        comment_state.set(input.value());
    });

    let on_submit = { 
        let product_id_state = product_id_state.clone();
        let rating_state = rating_state.clone();
        let comment_state = comment_state.clone();
        let loading_state = loading_state.clone();
        let error_message_state = error_message_state.clone();
        let on_submit_callback = props.on_submit.clone();
        let review = props.review.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            let product_id_str = (*product_id_state).clone();
            let rating_str = (*rating_state).clone();
            let comment = (*comment_state).clone();
            let loading_state = loading_state.clone();
            let error_message_state = error_message_state.clone();
            let on_submit_callback = on_submit_callback.clone();
            let review = review.clone();

            loading_state.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let product_id = match Uuid::parse_str(&product_id_str) {
                    Ok(id) => id,
                    Err(_) => {
                        error_message_state.set(Some("Invalid Product ID format.".to_string()));
                        loading_state.set(false);
                        return;
                    }
                };
                let rating = match rating_str.parse::<i32>() {
                    Ok(r) if r >= 1 && r <= 5 => r,
                    _ => {
                        error_message_state.set(Some("Rating must be between 1 and 5.".to_string()));
                        loading_state.set(false);
                        return;
                    }
                };

                let result = if let Some(existing_review) = review {
                    let update_dto = UpdateReview {
                        rating: Some(rating),
                        comment: Some(comment),
                    };
                    ReviewService::update_review(existing_review.id.to_string(), update_dto).await
                } else {
                    let create_review = CreateReview {
                        product_id,
                        user_id: Uuid::new_v4(), // This should be set by the backend based on auth
                        rating,
                        comment: Some(comment),
                    };
                    ReviewService::create_review(create_review).await
                };

                match result {
                    Ok(_) => {
                        log!("Review saved successfully");
                        error_message_state.set(None);
                        on_submit_callback.emit(());
                    },
                    Err(e) => {
                        log!("Failed to save review:", e.to_string());
                        error_message_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    let on_cancel = props.on_cancel.clone();

    html! {
        <div class="review-form">
            <h2>{ if props.review.is_some() { "Edit Review" } else { "Add New Review" } }</h2>
            <form onsubmit={on_submit}>
                <Input
                    label="Product ID"
                    id="product-id"
                    input_type="text"
                    value={(*product_id_state).clone()}
                    on_change={on_product_id_change}
                    placeholder="Product ID"
                    disabled={props.review.is_some()}
                />
                <Input
                    label="Rating (1-5)"
                    id="rating"
                    input_type="number"
                    value={(*rating_state).clone()}
                    on_change={on_rating_change}
                    placeholder="Rating"
                    min="1"
                    max="5"
                />
                <Input
                    label="Comment"
                    id="comment"
                    input_type="text"
                    value={(*comment_state).clone()}
                    on_change={on_comment_change}
                    placeholder="Your review comment"
                />
                { if *loading_state { html! { <Spinner /> } } else { html! {} } }
                { if let Some(msg) = &*error_message_state { html! { <p class="error-message">{ msg }</p> } } else { html! {} } }
                <Button label={ if props.review.is_some() { "Save Changes" } else { "Submit Review" } } button_input_type="submit" />
                <Button label="Cancel" button_input_type="button" onclick={on_cancel} />
            </form>
        </div>
    }
}

