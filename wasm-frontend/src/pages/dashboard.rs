use yew::prelude::*;

use crate::AppRoute;
use crate::components::Card;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub on_route_change: Callback<AppRoute>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let on_products_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| {
            on_route_change.emit(AppRoute::Products);
        })
    };

    let on_orders_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| {
            on_route_change.emit(AppRoute::Orders);
        })
    };

    let on_inventory_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| {
            on_route_change.emit(AppRoute::Inventory);
        })
    };

    let on_vendors_click = {
        let on_route_change = props.on_route_change.clone();
        Callback::from(move |_| {
            on_route_change.emit(AppRoute::Vendors);
        })
    };

    html! {
        <div class="dashboard-page">
            <div class="dashboard-header">
                <h1>{"داشبورد مدیریت"}</h1>
                <p>{"خوش آمدید به پنل مدیریت پلتفرم پما"}</p>
            </div>

            <div class="dashboard-summary">
                <div class="summary-cards">
                    <Card title="کل محصولات" class="summary-card">
                        <div class="summary-value">{"۱۲۳"}</div>
                    </Card>
                    
                    <Card title="سفارشات جدید" class="summary-card">
                        <div class="summary-value">{"۴۷"}</div>
                    </Card>
                    
                    <Card title="موجودی کم" class="summary-card">
                        <div class="summary-value">{"۵"}</div>
                    </Card>
                    
                    <Card title="فروش امروز" class="summary-card">
                        <div class="summary-value">{"۱۵,۲۰۰,۰۰۰ تومان"}</div>
                    </Card>
                </div>
            </div>

            <div class="dashboard-quick-actions">
                <h2>{"دسترسی سریع"}</h2>
                <div class="quick-action-grid">
                    <Card class="quick-action-card" onclick={on_products_click}>
                        <h3>{"مدیریت محصولات"}</h3>
                        <p>{"افزودن، ویرایش و حذف محصولات"}</p>
                    </Card>
                    <Card class="quick-action-card" onclick={on_orders_click}>
                        <h3>{"مدیریت سفارشات"}</h3>
                        <p>{"پیگیری و به‌روزرسانی وضعیت سفارشات"}</p>
                    </Card>
                    <Card class="quick-action-card" onclick={on_inventory_click}>
                        <h3>{"مدیریت موجودی"}</h3>
                        <p>{"بررسی و به‌روزرسانی انبار"}</p>
                    </Card>
                    <Card class="quick-action-card" onclick={on_vendors_click}>
                        <h3>{"مدیریت تامین‌کنندگان"}</h3>
                        <p>{"افزودن و مدیریت تامین‌کنندگان"}</p>
                    </Card>
                </div>
            </div>

            <div class="dashboard-recent-activity">
                <h2>{"فعالیت‌های اخیر"}</h2>
                <Card>
                    <ul>
                        <li>{"سفارش جدید #۱۲۳۴۵ توسط احمد رضایی ثبت شد."}<span class="activity-time">{"۵ دقیقه پیش"}</span></li>
                        <li>{"محصول 'گردآفرید کلاسیک' ویرایش شد."}<span class="activity-time">{"۱ ساعت پیش"}</span></li>
                        <li>{"موجودی 'گردآفرید مدرن' به ۱۰ عدد کاهش یافت."}<span class="activity-time">{"۳ ساعت پیش"}</span></li>
                        <li>{"سفارش #۱۲۳۴۴ به وضعیت 'تکمیل شده' تغییر یافت."}<span class="activity-time">{"دیروز"}</span></li>
                    </ul>
                </Card>
            </div>
        </div>
    }
}

