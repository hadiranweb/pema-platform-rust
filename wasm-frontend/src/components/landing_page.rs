use yew::prelude::*;
use models::{product::Product, user::User, order::Order, vendor::Vendor, pagination::PaginatedResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::i18n::components::Trans;


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
                                <Trans text_key={"platform_name".to_string()} />
                            </h1>
                        </div>
                        <nav class="hidden md:flex space-x-6 space-x-reverse">
                            <a href="#features" class="text-gray-600 hover:text-purple-600 transition-colors"><Trans text_key={"features".to_string()} /></a>
                            <a href="#stats" class="text-gray-600 hover:text-purple-600 transition-colors"><Trans text_key={"stats".to_string()} /></a>
                            <a href="#contact" class="text-gray-600 hover:text-purple-600 transition-colors"><Trans text_key={"contact".to_string()} /></a>
                        </nav>
                        <div class="flex space-x-2 space-x-reverse">
                            <button class="text-purple-600 hover:text-purple-700 px-4 py-2 rounded-lg transition-colors">
                                <Trans text_key={"login".to_string()} />
                            </button>
                            <button class="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg transition-colors">
                                <Trans text_key={"signup".to_string()} />
                            </button>
                        </div>
                    </div>
                </div>
            </header>

            // Hero Section
            <main class="py-20 px-4">
                <div class="container mx-auto text-center">
                    <h2 class="text-5xl md:text-6xl font-bold text-gray-900 mb-6 leading-tight">
                        <Trans text_key={"hero_title_part1".to_string()} />
                        <span class="block bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
                            <Trans text_key={"hero_title_part2".to_string()} />
                        </span>
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto leading-relaxed">
                        <Trans text_key={"hero_description".to_string()} />
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center mb-12">
                        <button class="bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white text-lg px-8 py-3 rounded-lg transform hover:scale-105 transition-all duration-200 shadow-lg">
                            <Trans text_key={"get_started".to_string()} />
                        </button>
                        <button class="border border-purple-200 hover:bg-purple-50 text-lg px-8 py-3 rounded-lg transform hover:scale-105 transition-all duration-200">
                            <Trans text_key={"view_demo".to_string()} />
                        </button>
                    </div>
                </div>
            </main>

            // Statistics Section
            <section id="stats" class="py-16 px-4 bg-white/50">
                <div class="container mx-auto">
                    <h3 class="text-3xl font-bold text-center text-gray-900 mb-12"><Trans text_key={"stats_title".to_string()} /></h3>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-purple-600 mb-2">{format!("{}+", stats.active_vendors)}</div>
                            <div class="text-gray-600"><Trans text_key={"active_vendors".to_string()} /></div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-blue-600 mb-2">{format!("{}K+", stats.total_products / 1000)}</div>
                            <div class="text-gray-600"><Trans text_key={"total_products".to_string()} /></div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-green-600 mb-2">{format!("{}K+", stats.daily_orders / 1000)}</div>
                            <div class="text-gray-600"><Trans text_key={"daily_orders".to_string()} /></div>
                        </div>
                        <div class="text-center p-6 bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-4xl font-bold text-orange-600 mb-2">{format!("{}%", stats.customer_satisfaction)}</div>
                            <div class="text-gray-600"><Trans text_key={"customer_satisfaction".to_string()} /></div>
                        </div>
                    </div>
                </div>
            </section>

            // Features Section
            <section id="features" class="py-16 px-4">
                <div class="container mx-auto">
                    <h3 class="text-3xl font-bold text-center text-gray-900 mb-12"><Trans text_key={"features_title".to_string()} /></h3>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"📦"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_product_management_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_product_management_description".to_string()} /></p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"🛍️"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_order_management_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_order_management_description".to_string()} /></p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"💳"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_payment_system_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_payment_system_description".to_string()} /></p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"📊"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_inventory_management_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_inventory_management_description".to_string()} /></p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"👥"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_vendor_management_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_vendor_management_description".to_string()} /></p>
                        </div>
                        <div class="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
                            <div class="text-3xl mb-4">{"🔔"}</div>
                            <h4 class="text-xl font-bold text-gray-900 mb-2"><Trans text_key={"feature_notification_system_title".to_string()} /></h4>
                            <p class="text-gray-600"><Trans text_key={"feature_notification_system_description".to_string()} /></p>
                        </div>
                    </div>
                </div>
            </section>

            // Technology Section
            <section class="py-16 px-4 bg-gray-50">
                <div class="container mx-auto text-center">
                    <h3 class="text-3xl font-bold text-gray-900 mb-8"><Trans text_key={"technology_title".to_string()} /></h3>
                    <p class="text-lg text-gray-600 mb-8 max-w-2xl mx-auto">
                        <Trans text_key={"technology_description".to_string()} />
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
                    <h3 class="text-3xl font-bold mb-4"><Trans text_key={"cta_title".to_string()} /></h3>
                    <p class="text-xl mb-8 opacity-90"><Trans text_key={"cta_description".to_string()} /></p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <button class="bg-white text-purple-600 hover:bg-gray-100 px-8 py-3 rounded-lg font-semibold transition-colors">
                            <Trans text_key={"cta_start_free".to_string()} />
                        </button>
                        <button class="border border-white hover:bg-white hover:text-purple-600 px-8 py-3 rounded-lg font-semibold transition-colors">
                            <Trans text_key={"cta_contact_sales".to_string()} />
                        </button>
                    </div>
                </div>
            </section>

            // Footer
            <footer id="contact" class="bg-gray-900 text-white py-12 px-4">
                <div class="container mx-auto">
                    <div class="grid md:grid-cols-4 gap-8">
                        <div>
                            <h4 class="text-lg font-bold mb-4"><Trans text_key={"platform_name".to_string()} /></h4>
                            <p class="text-gray-400"><Trans text_key={"footer_platform_description".to_string()} /></p>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4"><Trans text_key={"footer_products_title".to_string()} /></h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_product_store_management".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_product_payment_system".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_product_inventory_management".to_string()} /></a></li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4"><Trans text_key={"footer_support_title".to_string()} /></h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_support_help_center".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_support_contact_us".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_support_report_issue".to_string()} /></a></li>
                            </ul>
                        </div>
                        <div>
                            <h4 class="text-lg font-bold mb-4"><Trans text_key={"footer_company_title".to_string()} /></h4>
                            <ul class="space-y-2 text-gray-400">
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_company_about_us".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_company_blog".to_string()} /></a></li>
                                <li><a href="#" class="hover:text-white transition-colors"><Trans text_key={"footer_company_careers".to_string()} /></a></li>
                            </ul>
                        </div>
                    </div>
                    <div class="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
                        <p><Trans text_key={"footer_copyright".to_string()} /></p>
                    </div>
                </div>
            </footer>
        </div>
    }
}

