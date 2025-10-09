use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::components::{button::Button, card::Card};

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let navigator = use_navigator().unwrap();

    let on_view_products = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Products);
        })
    };

    let on_view_orders = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Orders);
        })
    };

    let on_view_inventory = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Inventory);
        })
    };

    let on_view_vendors = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Vendors);
        })
    };

    html! {
        <div class="dashboard-page">
            <div class="dashboard-header">
                <h1>{"داشبورد مدیریت"}</h1>
                <p>{"خوش آمدید به پنل مدیریت پلتفرم پما"}</p>
            </div>

            <div class="dashboard-stats">
                <Card title="کل محصولات" class="stat-card">
                    <div class="stat-value">{"۱۲۳"}</div>
                    <div class="stat-change positive">{"+۵.۲%"}</div>
                </Card>

                <Card title="سفارشات امروز" class="stat-card">
                    <div class="stat-value">{"۴۷"}</div>
                    <div class="stat-change positive">{"+۱۲.۳%"}</div>
                </Card>

                <Card title="موجودی کل" class="stat-card">
                    <div class="stat-value">{"۸۹۲"}</div>
                    <div class="stat-change negative">{"-۲.۱%"}</div>
                </Card>

                <Card title="تأمین‌کنندگان فعال" class="stat-card">
                    <div class="stat-value">{"۱۵"}</div>
                    <div class="stat-change neutral">{"بدون تغییر"}</div>
                </Card>
            </div>

            <div class="dashboard-actions">
                <h2>{"اقدامات سریع"}</h2>
                <div class="actions-grid">
                    <Card 
                        title="مدیریت محصولات" 
                        subtitle="مشاهده و ویرایش محصولات نقره‌ای"
                        class="action-card"
                        onclick={on_view_products}
                    >
                        <div class="action-icon">{"📦"}</div>
                    </Card>

                    <Card 
                        title="مدیریت سفارشات" 
                        subtitle="پیگیری و پردازش سفارشات"
                        class="action-card"
                        onclick={on_view_orders}
                    >
                        <div class="action-icon">{"📋"}</div>
                    </Card>

                    <Card 
                        title="مدیریت موجودی" 
                        subtitle="کنترل موجودی انبار"
                        class="action-card"
                        onclick={on_view_inventory}
                    >
                        <div class="action-icon">{"📊"}</div>
                    </Card>

                    <Card 
                        title="مدیریت تأمین‌کنندگان" 
                        subtitle="ارتباط با تأمین‌کنندگان"
                        class="action-card"
                        onclick={on_view_vendors}
                    >
                        <div class="action-icon">{"🤝"}</div>
                    </Card>
                </div>
            </div>

            <div class="dashboard-recent">
                <h2>{"فعالیت‌های اخیر"}</h2>
                <Card class="recent-activity">
                    <div class="activity-list">
                        <div class="activity-item">
                            <span class="activity-time">{"۱۰:۳۰"}</span>
                            <span class="activity-text">{"سفارش جدید از مشتری احمد رضایی"}</span>
                        </div>
                        <div class="activity-item">
                            <span class="activity-time">{"۰۹:۱۵"}</span>
                            <span class="activity-text">{"محصول گردآفرید طلایی به موجودی اضافه شد"}</span>
                        </div>
                        <div class="activity-item">
                            <span class="activity-time">{"۰۸:۴۵"}</span>
                            <span class="activity-text">{"تأمین‌کننده جدید ثبت شد"}</span>
                        </div>
                        <div class="activity-item">
                            <span class="activity-time">{"۰۸:۰۰"}</span>
                            <span class="activity-text">{"گزارش فروش ماهانه تولید شد"}</span>
                        </div>
                    </div>
                </Card>
            </div>

            <div class="dashboard-charts">
                <h2>{"نمودارهای عملکرد"}</h2>
                <div class="charts-grid">
                    <Card title="فروش ماهانه" class="chart-card">
                        <div class="chart-placeholder">
                            {"نمودار فروش ماهانه"}
                            <br />
                            {"(نمودار واقعی با Recharts پیاده‌سازی خواهد شد)"}
                        </div>
                    </Card>

                    <Card title="محبوب‌ترین محصولات" class="chart-card">
                        <div class="chart-placeholder">
                            {"نمودار محصولات پرفروش"}
                            <br />
                            {"(نمودار واقعی با Recharts پیاده‌سازی خواهد شد)"}
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
