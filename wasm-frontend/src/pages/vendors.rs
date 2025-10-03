use yew::prelude::*;
use crate::components::{button::Button, card::Card, input::Input};

#[function_component(Vendors)]
pub fn vendors() -> Html {
    html! {
        <div class="vendors-page">
            <div class="vendors-header">
                <h1>{"مدیریت تأمین‌کنندگان"}</h1>
                <p>{"مدیریت ارتباط با تأمین‌کنندگان"}</p>
            </div>
            <Card>
                <p>{"صفحه مدیریت تأمین‌کنندگان در حال توسعه است..."}</p>
            </Card>
        </div>
    }
}
