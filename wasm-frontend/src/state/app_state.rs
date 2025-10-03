use yew::prelude::*;
use std::rc::Rc;
use models::user::User;
use super::{AuthState, ProductState, OrderState};

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub auth: AuthState,
    pub products: ProductState,
    pub orders: OrderState,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            auth: AuthState::default(),
            products: ProductState::default(),
            orders: OrderState::default(),
            loading: false,
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppAction {
    SetLoading(bool),
    SetError(Option<String>),
    Auth(super::AuthAction),
    Product(super::ProductAction),
    Order(super::OrderAction),
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::SetLoading(loading) => Rc::new(AppState {
                loading,
                ..(*self).clone()
            }),
            AppAction::SetError(error) => Rc::new(AppState {
                error,
                ..(*self).clone()
            }),
            AppAction::Auth(auth_action) => {
                let new_auth = self.auth.clone().reduce(auth_action);
                Rc::new(AppState {
                    auth: new_auth,
                    ..(*self).clone()
                })
            },
            AppAction::Product(product_action) => {
                let new_products = self.products.clone().reduce(product_action);
                Rc::new(AppState {
                    products: new_products,
                    ..(*self).clone()
                })
            },
            AppAction::Order(order_action) => {
                let new_orders = self.orders.clone().reduce(order_action);
                Rc::new(AppState {
                    orders: new_orders,
                    ..(*self).clone()
                })
            },
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;
