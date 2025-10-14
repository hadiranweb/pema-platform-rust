use yew::prelude::*;
use gloo_console::log;
use yew_router::prelude::*;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::review_service::{ReviewService, Review};
use crate::router::routes::AppRoute;

#[function_component(ReviewListPage)]
pub fn review_list_page() -> Html {
    let reviews_state = use_state(|| None::<Vec<Review>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let fetch_reviews = { 
        let reviews_state = reviews_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let reviews_state = reviews_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match ReviewService::get_my_reviews().await {
                    Ok(reviews) => {
                        reviews_state.set(Some(reviews));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch reviews:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_reviews| {
        fetch_reviews.emit(());
        || ()
    }, fetch_reviews.clone());

    let on_delete_review = { 
        let fetch_reviews = fetch_reviews.clone();
        Callback::from(move |review_id: String| {
            let fetch_reviews = fetch_reviews.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ReviewService::delete_review(review_id).await {
                    Ok(_) => {
                        log!("Review deleted successfully");
                        fetch_reviews.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete review:", e.to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="review-list-page">
            <h1>{ "My Reviews" }</h1>
            <Link<AppRoute> to={AppRoute::ReviewForm { id: None }}>{ "Add New Review" }</Link<AppRoute>>

            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            
            <div class="review-list">
                { 
                    if let Some(reviews) = &*reviews_state {
                        if reviews.is_empty() {
                            html! { <p>{ "You haven't posted any reviews yet." }</p> }
                        } else {
                            reviews.iter().map(|review| {
                                let review_id = review.id.to_string();
                                let on_delete = on_delete_review.clone();
                                html! {
                                    <div key={review_id.clone()} class="review-card">
                                        <h3>{ format!("Product ID: {}", review.product_id) }</h3>
                                        <p>{ format!("Rating: {}/5", review.rating) }</p>
                                        <p>{ format!("Comment: {}", review.comment.as_deref().unwrap_or("N/A")) }</p>
                                        <Link<AppRoute> to={AppRoute::ReviewForm { id: Some(review_id.clone()) }}>{ "Edit" }</Link<AppRoute>>
                                        <button onclick={move |_| on_delete.emit(review_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading reviews..." }</p> }
                    }
                }
            </div>
        </div>
    }
}

