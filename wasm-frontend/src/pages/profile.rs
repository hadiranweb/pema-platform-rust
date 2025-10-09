use yew::prelude::*;
use crate::components::{button::Button, card::Card, input::Input};

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div class="profile-page">
            <div class="profile-header">
                <h1>{"پروفایل کاربری"}</h1>
                <p>{"مدیریت اطلاعات حساب کاربری"}</p>
            </div>
            <Card>
                <p>{"صفحه پروفایل کاربری در حال توسعه است..."}</p>
            </Card>
        </div>
    }
}
