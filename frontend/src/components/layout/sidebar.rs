use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <aside class="sidebar">
            <nav>
                <ul>
                    <li><a href="/admin/dashboard">{ "Admin Dashboard" }</a></li>
                    <li><a href="/admin/users">{ "Users" }</a></li>
                    <li><a href="/admin/products">{ "Products" }</a></li>
                    <li><a href="/admin/orders">{ "Orders" }</a></li>
                    <li><a href="/admin/vendors">{ "Vendors" }</a></li>
                    <li><a href="/admin/pages">{ "Pages" }</a></li>
                </ul>
            </nav>
        </aside>
    }
}

