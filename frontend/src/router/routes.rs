use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::admin::{AdminDashboardPage, AdminUsersPage, AdminVendorsPage, AdminProductsPage, AdminOrdersPage, AdminPagesPage};
use crate::pages::auth::{LoginPage, RegisterPage};
use crate::pages::dashboard::DashboardPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::orders::{OrdersListPage, OrderDetailPage};
use crate::pages::products::{ProductsPage, ProductDetailPage};
use crate::pages::profile::ProfilePage;
use crate::pages::vendor::{VendorDashboardPage, VendorProductsPage, VendorOrdersPage};
use crate::pages::wallet::WalletPage;
use crate::pages::reviews::{ReviewListPage, ReviewFormPage};
use crate::pages::pages::{PagesListPage, PageDetailPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
    #[at("/profile")]
    Profile,
    #[at("/products")]
    Products,
    #[at("/products/:id")]
    ProductDetail { id: String },
    #[at("/orders")]
    Orders,
    #[at("/orders/:id")]
    OrderDetail { id: String },
    #[at("/wallet")]
    Wallet,
    #[at("/reviews")]
    Reviews,
    #[at("/reviews/new")]
    ReviewForm,
    #[at("/reviews/edit/:id")]
    ReviewEdit { id: String },
    #[at("/admin/dashboard")]
    AdminDashboard,
    #[at("/admin/users")]
    AdminUsers,
    #[at("/admin/vendors")]
    AdminVendors,
    #[at("/admin/products")]
    AdminProducts,
    #[at("/admin/orders")]
    AdminOrders,
    #[at("/admin/pages")]
    AdminPages,
    #[at("/vendor/dashboard")]
    VendorDashboard,
    #[at("/vendor/products")]
    VendorProducts,
    #[at("/vendor/orders")]
    VendorOrders,
    #[at("/pages")]
    PagesList,
    #[at("/pages/:id")]
    PageDetail { id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::Register => html! { <RegisterPage /> },
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::Products => html! { <ProductsPage /> },
        Route::ProductDetail { id } => html! { <ProductDetailPage id={id} /> },
        Route::Orders => html! { <OrdersListPage /> },
        Route::OrderDetail { id } => html! { <OrderDetailPage id={id} /> },
        Route::Wallet => html! { <WalletPage /> },
        Route::Reviews => html! { <ReviewListPage /> },
        Route::ReviewForm => html! { <ReviewFormPage /> },
        Route::ReviewEdit { id } => html! { <ReviewFormPage id={Some(id)} /> },
        Route::AdminDashboard => html! { <AdminDashboardPage /> },
        Route::AdminUsers => html! { <AdminUsersPage /> },
        Route::AdminVendors => html! { <AdminVendorsPage /> },
        Route::AdminProducts => html! { <AdminProductsPage /> },
        Route::AdminOrders => html! { <AdminOrdersPage /> },
        Route::AdminPages => html! { <AdminPagesPage /> },
        Route::VendorDashboard => html! { <VendorDashboardPage /> },
        Route::VendorProducts => html! { <VendorProductsPage /> },
        Route::VendorOrders => html! { <VendorOrdersPage /> },
        Route::PagesList => html! { <PagesListPage /> },
        Route::PageDetail { id } => html! { <PageDetailPage id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

