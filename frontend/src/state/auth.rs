use yewdux::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct AuthStore {
    pub token: Option<String>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl Reducer for AuthStore {
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Action::Login(token, user_id, username, email) => Self {
                token: Some(token),
                user_id: Some(user_id),
                username: Some(username),
                email: Some(email),
            }.into(),
            Action::Logout => Self {
                token: None,
                user_id: None,
                username: None,
                email: None,
            }.into(),
        }
    }
}

pub enum Action {
    Login(String, Uuid, String, String),
    Logout,
}

