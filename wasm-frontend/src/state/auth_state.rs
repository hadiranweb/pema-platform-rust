use models::user::User;
use crate::services::auth::AuthResponse;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user: Option<User>,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub is_authenticated: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            token: None,
            refresh_token: None,
            is_authenticated: false,
            is_loading: false,
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthAction {
    LoginStart,
    LoginSuccess(AuthResponse),
    LoginFailure(String),
    Logout,
    SetUser(User),
    SetLoading(bool),
    SetError(Option<String>),
    RefreshTokenSuccess(AuthResponse),
    RefreshTokenFailure(String),
}

impl AuthState {
    pub fn reduce(self, action: AuthAction) -> Self {
        match action {
            AuthAction::LoginStart => Self {
                is_loading: true,
                error: None,
                ..self
            },
            AuthAction::LoginSuccess(auth_response) => Self {
                user: Some(auth_response.user),
                token: Some(auth_response.token),
                refresh_token: Some(auth_response.refresh_token),
                is_authenticated: true,
                is_loading: false,
                error: None,
                ..self
            },
            AuthAction::LoginFailure(error) => Self {
                user: None,
                token: None,
                refresh_token: None,
                is_authenticated: false,
                is_loading: false,
                error: Some(error),
                ..self
            },
            AuthAction::Logout => Self {
                user: None,
                token: None,
                refresh_token: None,
                is_authenticated: false,
                is_loading: false,
                error: None,
                ..self
            },
            AuthAction::SetUser(user) => Self {
                user: Some(user),
                ..self
            },
            AuthAction::SetLoading(is_loading) => Self {
                is_loading,
                ..self
            },
            AuthAction::SetError(error) => Self {
                error,
                ..self
            },
            AuthAction::RefreshTokenSuccess(auth_response) => Self {
                user: Some(auth_response.user),
                token: Some(auth_response.token),
                refresh_token: Some(auth_response.refresh_token),
                is_authenticated: true,
                is_loading: false,
                error: None,
                ..self
            },
            AuthAction::RefreshTokenFailure(error) => Self {
                user: None,
                token: None,
                refresh_token: None,
                is_authenticated: false,
                is_loading: false,
                error: Some(error),
                ..self
            },
        }
    }
}
