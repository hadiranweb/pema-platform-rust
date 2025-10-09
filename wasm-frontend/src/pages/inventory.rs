use yew::prelude::*;
use crate::components::{button::Button, card::Card, input::Input};

#[function_component(Inventory)]
pub fn inventory() -> Html {
    html! {
        <div class="inventory-page">
            <div class="inventory-header">
                <h1>{"مدیریت موجودی"}</h1>
                <p>{"کنترل و مدیریت موجودی انبار"}</p>
            </div>
            <Card>
                <p>{"صفحه مدیریت موجودی در حال توسعه است..."}</p>
            </Card>
        </div>
    }
}
