use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminDashboardProps {}

#[function_component(AdminDashboard)]
pub fn admin_dashboard(_props: &AdminDashboardProps) -> Html {
    html! {
        <div class="min-h-screen bg-gray-50" dir="rtl">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold text-gray-900 mb-8">{"داشبورد مدیریت"}</h1>
                <p class="text-gray-600">{"این بخش در مرحله بعدی پیاده‌سازی خواهد شد."}</p>
            </div>
        </div>
    }
}
