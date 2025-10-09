use yew::prelude::*;
use crate::components::Card;

#[function_component(Inventory)]
pub fn inventory() -> Html {
    html! {
        <div class="inventory-page">
            <div class="inventory-header">
                <h1>{"مدیریت موجودی"}</h1>
                <p>{"بررسی و به‌روزرسانی موجودی انبار"}</p>
            </div>

            <div class="inventory-summary">
                <div class="summary-cards">
                    <Card title="کل اقلام" class="summary-card">
                        <div class="summary-value">{"۲۵۰"}</div>
                    </Card>
                    
                    <Card title="اقلام با موجودی کم" class="summary-card">
                        <div class="summary-value">{"۱۵"}</div>
                    </Card>
                    
                    <Card title="اقلام در انتظار" class="summary-card">
                        <div class="summary-value">{"۸"}</div>
                    </Card>
                </div>
            </div>

            <div class="inventory-list">
                <h2>{"لیست موجودی"}</h2>
                <Card>
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>{"نام محصول"}</th>
                                <th>{"موجودی"}</th>
                                <th>{"وضعیت"}</th>
                                <th>{"آخرین به‌روزرسانی"}</th>
                                <th>{"عملیات"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"گردآفرید کلاسیک"}</td>
                                <td>{"۲۵"}</td>
                                <td><span class="status-badge stock-good">{"کافی"}</span></td>
                                <td>{"۱۴۰۳/۰۸/۱۵"}</td>
                                <td class="actions-cell">
                                    <button class="button secondary small">{"ویرایش"}</button>
                                </td>
                            </tr>
                            <tr>
                                <td>{"گردآفرید مدرن"}</td>
                                <td>{"۱۰"}</td>
                                <td><span class="status-badge stock-low">{"کم"}</span></td>
                                <td>{"۱۴۰۳/۰۸/۱۴"}</td>
                                <td class="actions-cell">
                                    <button class="button secondary small">{"ویرایش"}</button>
                                </td>
                            </tr>
                            <tr>
                                <td>{"گردآفرید لوکس"}</td>
                                <td>{"۵"}</td>
                                <td><span class="status-badge stock-out">{"بسیار کم"}</span></td>
                                <td>{"۱۴۰۳/۰۸/۱۳"}</td>
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

