use yew::prelude::*;
use models::{product::Product, user::User, order::Order, vendor::Vendor, pagination::PaginatedResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformStats {
    pub active_vendors: u32,
    pub total_products: u32,
    pub daily_orders: u32,
    pub customer_satisfaction: u32,
}

impl Default for PlatformStats {
    fn default() -> Self {
        Self {
            active_vendors: 500,
            total_products: 10000,
            daily_orders: 1000,
            customer_satisfaction: 98,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct LandingPageProps {
    #[prop_or_default]
    pub stats: PlatformStats,
}

#[function_component(LandingPage)]
pub fn landing_page(props: &LandingPageProps) -> Html {
    let stats = &props.stats;

    html! {
        <div class="min-h-screen bg-gradient-to-br from-purple-50 via-blue-50 to-indigo-100" dir="rtl">
            // Header Section
            <header class="bg-white/80 backdrop-blur-md border-b border-gray-200 sticky top-0 z-50">
                <div class="container mx-auto px-4 py-4">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-4 space-x-reverse">
                            <div class="bg-gradient-to-r from-purple-600 to-blue-600 text-white p-2 rounded-lg">
                                <span class="text-xl">{"🛒"}</span>
                            </div>
                            <h1 class="text-2xl font-bold bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
                                {"پلتفرم پما"}
                            </h1>
                        </div>
                        <nav class="hidden md:flex space-x-6 space-x-reverse">
                            <a href="#features" class="text-gray-600 hover:text-purple-600 transition-colors">{"ویژگی‌ها"}</a>
                            <a href="#stats" class="text-gray-600 hover:text-purple-600 transition-colors">{"آمار"}</a>
                            <a href="#contact" class="text-gray-600 hover:text-purple-600 transition-colors">{"تماس"}</a>
                        </nav>
                        <div class="flex space-x-2 space-x-reverse">
                            <button class="text-purple-600 hover:text-purple-700 px-4 py-2 rounded-lg transition-colors">
                                {"ورود"}
                            </button>
                            <button class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg transition-colors">
                                {"ثبت نام"}
                            </button>
                        </div>
                    </div>
                </div>
            </header>

            // Hero Section
            <main class="py-20 px-4">
                <div class="container mx-auto text-center">
                    <h2 class="text-5xl md:text-6xl font-bold text-gray-900 mb-6 leading-tight">
                        {"پلتفرم تجارت الکترونیک"}
                        <span class="block bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
                            {"نسل جدید"}
                        </span>
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto leading-relaxed">
                        {"سیستم جامع مدیریت فروشگاه آنلاین با قابلیت‌های پیشرفته برای فروشندگان و مشتریان. مدیریت محصولات، سفارشات، پرداخت‌ها و موجودی با تکنولوژی WebAssembly."}
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mb-12">
                        <button class="bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white text-lg px-8 py-3 rounded-lg transform hover:scale-105 transition-all duration-200 shadow-lg">
                            {"شروع کنید"}
                        </button>
                        <button class="border border-purple-200 hover:bg-purple-50 text-lg px-8 py-3 rounded-lg transform hover:scale-105 transition-all duration-200">
                            {"مشاهده دمو"}
                        </button>
                    </div>
                </div>
            </main>

            // Statistics Section
            <section id="stats" class="py-16 px-4 bg-white/50">
                <div class="container mx-auto">
                    <h3 class="text-3xl font-bold text-center text-gray-900 mb-12">{"آمار پلتفرم"}</h3>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-purple-600 mb-2">{format!("{}+", stats.active_vendors)}</div>
                            <div class="text-gray-600">{"فروشندگان فعال"}</div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-blue-600 mb-2">{format!("{}K+", stats.total_products / 1000)}</div>
                            <div class="text-gray-600">{"محصولات"}</div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-green-600 mb-2">{format!("{}K+", stats.daily_orders / 1000)}</div>
                            <div class="text-gray-600">{"سفارشات روزانه"}</div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-orange-600 mb-2">{format!("{}%", stats.customer_satisfaction)}</div>
                            <div class="text-gray-600">{"رضایت مشتریان"}</div>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section id="features" class="py-16 px-4">
                <div class="container mx-auto">
                    <h3 class="text-3xl font-bold text-center text-gray-900 mb-12">{"ویژگی‌های کلیدی"}</h3>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"📦"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"مدیریت محصولات"}</h4>
                            <p class="text-gray-600">{"مدیریت کامل محصولات شامل ایجاد، ویرایش، دسته‌بندی و تنظیم قیمت‌ها با رابط کاربری ساده و کارآمد."}</p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"🛍️"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"مدیریت سفارشات"}</h4>
                            <p class="text-gray-600">{"پردازش و پیگیری سفارشات از ثبت تا تحویل با امکان مدیریت وضعیت‌ها و ارسال اطلاع‌رسانی‌ها."}</p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"💳"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"سیستم پرداخت"}</h4>
                            <p class="text-gray-600">{"پردازش امن پرداخت‌ها با پشتیبانی از روش‌های مختلف پرداخت و مدیریت تراکنش‌ها."}</p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"📊"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"مدیریت موجودی"}</h4>
                            <p class="text-gray-600">{"کنترل دقیق موجودی محصولات با هشدارهای خودکار و مدیریت انبارها."}</p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"👥"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"مدیریت فروشندگان"}</h4>
                            <p class="text-gray-600">{"سیستم جامع مدیریت فروشندگان شامل ثبت‌نام، تأیید و نظارت بر عملکرد."}</p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"🔔"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2">{"سیستم اطلاع‌رسانی"}</h4>
                            <p class="text-gray-600">{"ارسال اطلاع‌رسانی‌های لحظه‌ای و مدیریت پیام‌ها برای کاربران و فروشندگان."}</p>
                        </div>
                    </div>
                </div>
            </section>

            // Technology Section
            <section class="py-16 px-4 bg-gray-50">
                <div class="container mx-auto text-center">
                    <h3 class="text-3xl font-bold text-gray-900 mb-8">{"تکنولوژی پیشرفته"}</h3>
                    <p class="text-lg text-gray-600 mb-8 max-w-2xl mx-auto">
                        {"پلتفرم پما با استفاده از تکنولوژی‌های مدرن Rust و WebAssembly ساخته شده است که عملکرد بالا، امنیت و قابلیت اطمینان را تضمین می‌کند."}
                    </p>
                    <div class="flex flex-wrap justify-center gap-4">
                        <span class="bg-orange-100 text-orange-800 px-4 py-2 rounded-full">{"Rust"}</span>
                        <span class="bg-blue-100 text-blue-800 px-4 py-2 rounded-full">{"WebAssembly"}</span>
                        <span class="bg-green-100 text-green-800 px-4 py-2 rounded-full">{"Yew Framework"}</span>
                        <span class="bg-purple-100 text-purple-800 px-4 py-2 rounded-full">{"Type Safety"}</span>
                        <span class="bg-red-100 text-red-800 px-4 py-2 rounded-full">{"High Performance"}</span>
                    </div>
                </div>
            </section>

            // Call to Action Section
            <section class="py-16 px-4 bg-gradient-to-r from-purple-600 to-blue-600 text-white">
                <div class="container mx-auto text-center">
                    <h3 class="text-3xl font-bold mb-4">{"آماده شروع هستید؟"}</h3>
                    <p class="text-xl mb-8 opacity-90">{"همین امروز فروشگاه آنلاین خود را راه‌اندازی کنید"}</p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <button class="bg-white text-purple-600 hover:bg-gray-100 px-8 py-3 rounded-lg font-semibold transition-colors">
                            {"شروع رایگان"}</button>
                        <button class="border border-white hover:bg-white hover:text-purple-600 px-8 py-3 rounded-lg font-semibold transition-colors">
                            {"تماس با فروش"}
                        </button>
                    </div>
                </div>
            </section>

            // Footer
            <footer id="contact" class="bg-gray-900 text-white py-12 px-4">
                <div class="container mx-auto">
                    <div class="grid md:grid-cols-4 gap-8">
                        <div>
                            <h4 class="text-lg font-bold mb-4">{"پلتفرم پما"}</h4>
                            <p class="text-gray-400">{"راه‌حل جامع تجارت الکترونیک برای کسب‌وکارهای مدرن"}</p>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4">{"محصولات"}</h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors">{"مدیریت فروشگاه"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"سیستم پرداخت"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"مدیریت موجودی"}</a></li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4">{"پشتیبانی"}</h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors">{"مرکز راهنمایی"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"تماس با ما"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"گزارش مشکل"}</a></li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4">{"شرکت"}</h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors">{"درباره ما"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"وبلاگ"}</a></li>
                                <li><a href="#" class="hover:text-white transition-colors">{"فرصت‌های شغلی"}</a></li>
                            </ul>
                        </div>
                    </div>
                    <div class="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
                        <p>{"© 2024 پلتفرم پما. تمامی حقوق محفوظ است."}</p>
                    </div>
                </div>
            </footer>
        </div>
    }
}
