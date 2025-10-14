
use yew::prelude::*;
use gloo_console::log;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::services::admin_service::{AdminService, User, AdminUserUpdateDto};

#[function_component(AdminUsersPage)]
pub fn admin_users_page() -> Html {
    let users_state = use_state(|| None::<Vec<User>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let fetch_users = { 
        let users_state = users_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let users_state = users_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match AdminService::get_all_users().await {
                    Ok(users) => {
                        users_state.set(Some(users));
                        error_state.set(None);
                    },
                    Err(e) => {
                        log!("Failed to fetch users:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_users| {
        fetch_users.emit(());
        || ()
    }, fetch_users.clone());

    let on_delete_user = { 
        let fetch_users = fetch_users.clone();
        Callback::from(move |user_id: String| {
            let fetch_users = fetch_users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match AdminService::delete_user(user_id).await {
                    Ok(_) => {
                        log!("User deleted successfully");
                        fetch_users.emit(());
                    },
                    Err(e) => {
                        log!("Failed to delete user:", e.to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="admin-users-page">
            <h1>{ "Manage Users" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }
            <div class="user-list">
                { 
                    if let Some(users) = &*users_state {
                        if users.is_empty() {
                            html! { <p>{ "No users found." }</p> }
                        } else {
                            users.iter().map(|user| {
                                let user_id = user.id.to_string();
                                let on_delete = on_delete_user.clone();
                                html! {
                                    <div key={user_id.clone()} class="user-card">
                                        <h3>{ &user.username }</h3>
                                        <p>{ &user.email }</p>
                                        <p>{ format!("Role: {}", user.role) }</p>
                                        <button onclick={move |_| on_delete.emit(user_id.clone())}>{ "Delete" }</button>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! { <p>{ "Loading users..." }</p> }
                    }
                }
            </div>
        </div>
    }
}

