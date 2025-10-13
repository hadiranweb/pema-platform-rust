use yew::prelude::*;
use crate::components::Card;

#[function_component(Vendors)]
pub fn vendors() -> Html {
    html! {
        <div class="vendors-page">
            <div class="vendors-header">
                <h1>{"مدیریت تامین‌کنندگان"}</h1>
                <p>{"افزودن و مدیریت تامین‌کنندگان پلتفرم پما"}</p>
            </div>

            <div class="vendors-summary">
                <div class="summary-cards">
                    <Card title="کل تامین‌کنندگان" class="summary-card">
                        <div class="summary-value">{"۲۰"}</div>
                    </Card>
                    
                    <Card title="تامین‌کنندگان فعال" class="summary-card">
                        <div class="summary-value">{"۱۸"}</div>
                    </Card>
                    
                    <Card title="تامین‌کنندگان جدید (ماه جاری)" class="summary-card">
                        <div class="summary-value">{"۳"}</div>
                    </Card>
                </div>
            </div>

            <div class="vendors-list">
                <h2>{"لیست تامین‌کنندگان"}</h2>
                <Card>
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>{"نام تامین‌کننده"}</th>
                                <th>{"مسئول تماس"}</th>
                                <th>{"شماره تماس"}</th>
                                <th>{"ایمیل"}</th>
                                <th>{"وضعیت"}</th>
                                <th>{"عملیات"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"شرکت نقره‌سازان پارس"}</td>
                                <td>{"علی حسینی"}</td>
                                <td>{"۰۹۱۲۳۴۵۶۷۸۹"}</td>
                                <td>{"ali.h@silverpars.com"}</td>
                                <td><span class="status-badge status-active">{"فعال"}</span></td>
                                <td class="actions-cell">
                                    <button class="button secondary small">{"ویرایش"}</button>
                                </td>
                            </tr>
                            <tr>
                                <td>{"گالری نقره آریایی"}</td>
                                <td>{"سارا محمدی"}</td>
                                <td>{"۰۹۳۵۱۲۳۴۵۶۷"}</td>
                                <td>{"sara.m@ariasilver.com"}</td>
                                <td><span class="status-badge status-active">{"فعال"}</span></td>
                                <td class="actions-cell">
                                    <button class="button secondary small">{"ویرایش"}</button>
                                </td>
                            </tr>
                            <tr>
                                <td>{"کارگاه زرگری الماس"}</td>
                                <td>{"رضا کریمی"}</td>
                                <td>{"۰۹۱۹۸۷۶۵۴۳۲"}</td>
                                <td>{"reza.k@almasgold.com"}</td>
                                <td><span class="status-badge status-inactive">{"غیرفعال"}</span></td>
                                <td class="actions-cell">
                                    <button class="button secondary small">{"ویرایش"}</button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </Card>
            </div>
        </div>
    }
}

