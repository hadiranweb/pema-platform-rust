use yew::prelude::*;
use crate::components::Card;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div class="profile-page">
            <div class="profile-header">
                <h1>{"پروفایل کاربری"}</h1>
                <p>{"مدیریت اطلاعات شخصی و تنظیمات حساب"}</p>
            </div>

            <div class="profile-details">
                <Card title="اطلاعات شخصی" class="profile-card">
                    <div class="detail-item">
                        <span class="detail-label">{"نام و نام خانوادگی:"}</span>
                        <span class="detail-value">{"جان دو"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"ایمیل:"}</span>
                        <span class="detail-value">{"john.doe@example.com"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"شماره تماس:"}</span>
                        <span class="detail-value">{"۰۹۱۲۳۴۵۶۷۸۹"}</span>
                    </div>
                    <div class="detail-item">
                        <span class="detail-label">{"نقش:"}</span>
                        <span class="detail-value">{"مدیر سیستم"}</span>
                    </div>
                </Card>

                <Card title="تنظیمات حساب" class="profile-card">
                    <div class="setting-item">
                        <span class="setting-label">{"تغییر رمز عبور"}</span>
                        <button class="button secondary small">{"تغییر"}</button>
                    </div>
                    <div class="setting-item">
                        <span class="setting-label">{"فعال‌سازی احراز هویت دو مرحله‌ای"}</span>
                        <button class="button secondary small">{"فعال‌سازی"}</button>
                    </div>
                    <div class="setting-item">
                        <span class="setting-label">{"مدیریت اعلان‌ها"}</span>
                        <button class="button secondary small">{"تنظیمات"}</button>
                    </div>
                </Card>
            </div>
        </div>
    }
}

