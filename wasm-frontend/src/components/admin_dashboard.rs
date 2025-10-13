use yew::prelude::*;
use models::{
    product::Product, 
    order::Order, 
    user::User, 
    vendor::Vendor, 
    inventory::InventoryItem,
    notification::Notification,
    pagination::PaginatedResponse
};
use crate::components::{
    ProductCard, OrderItem, UserProfile, VendorCard, 
    NotificationItem, PaginationComponent, StatsWidget,
    ProductManagement, OrderManagement
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DashboardTab {
    Overview,
    Products,
    Orders,
    Users,
    Vendors,
    Inventory,
    Notifications,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_products: u32,
    pub total_orders: u32,
    pub total_users: u32,
    pub total_vendors: u32,
    pub pending_orders: u32,
    pub low_stock_items: u32,
    pub unread_notifications: u32,
    pub monthly_revenue: f64,
}

impl Default for DashboardStats {
    fn default() -> Self {
        Self {
            total_products: 1250,
            total_orders: 3420,
            total_users: 8900,
            total_vendors: 156,
            pending_orders: 23,
            low_stock_items: 12,
            unread_notifications: 5,
            monthly_revenue: 125000.0,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminDashboardProps {
    #[prop_or_default]
    pub stats: DashboardStats,
}

#[function_component(AdminDashboard)]
pub fn admin_dashboard(props: &AdminDashboardProps) -> Html {
    let active_tab = use_state(|| DashboardTab::Overview);
    let stats = &props.stats;

    let handle_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: DashboardTab| {
            active_tab.set(tab);
        })
    };

    let sidebar_items = vec![
        (DashboardTab::Overview, "📊", "نمای کلی"),
        (DashboardTab::Products, "📦", "محصولات"),
        (DashboardTab::Orders, "🛍️", "سفارشات"),
        (DashboardTab::Users, "👥", "کاربران"),
        (DashboardTab::Vendors, "🏪", "فروشندگان"),
        (DashboardTab::Inventory, "📋", "موجودی"),
        (DashboardTab::Notifications, "🔔", "اطلاع‌رسانی"),
        (DashboardTab::Settings, "⚙️", "تنظیمات"),
    ];

    html! {
        <div class="min-h-screen bg-gray-50" dir="rtl">
            // Header
            <header class="bg-white shadow-sm border-b border-gray-200">
                <div class="px-6 py-4">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-4 space-x-reverse">
                            <div class="bg-gradient-to-r from-purple-600 to-blue-600 text-white p-2 rounded-lg">
                                <span class="text-xl">{"🛒"}</span>
                            </div>
                            <div>
                                <h1 class="text-2xl font-bold text-gray-900">{"داشبورد مدیریت پما"}</h1>
                                <p class="text-sm text-gray-500">{"پنل مدیریت جامع پلتفرم تجارت الکترونیک"}</p>
                            </div>
                        </div>
                        <div class="flex items-center space-x-4 space-x-reverse">
                            <button class="relative p-2 text-gray-600 hover:text-purple-600 transition-colors">
                                <span class="text-xl">{"🔔"}</span>
                                if stats.unread_notifications > 0 {
                                    <span class="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full h-5 w-5 flex items-center justify-center">
                                        {stats.unread_notifications}
                                    </span>
                                }
                            </button>
                            <div class="flex items-center space-x-2 space-x-reverse">
                                <div class="w-8 h-8 bg-purple-600 rounded-full flex items-center justify-center text-white text-sm font-semibold">
                                    {"م"}
                                </div>
                                <span class="text-sm font-medium text-gray-700">{"مدیر سیستم"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </header>

            <div class="flex">
                // Sidebar
                <aside class="w-64 bg-white shadow-sm min-h-screen">
                    <nav class="p-4">
                        <ul class="space-y-2">
                            {for sidebar_items.iter().map(|(tab, icon, label)| {
                                let is_active = *active_tab == *tab;
                                let tab_clone = tab.clone();
                                let handle_click = {
                                    let handle_tab_change = handle_tab_change.clone();
                                    Callback::from(move |_| {
                                        handle_tab_change.emit(tab_clone.clone());
                                    })
                                };
                                
                                html! {
                                    <li>
                                        <button
                                            onclick={handle_click}
                                            class={if is_active {
                                                "w-full flex items-center space-x-3 space-x-reverse px-4 py-3 text-purple-600 bg-purple-50 rounded-lg font-medium transition-colors"
                                            } else {
                                                "w-full flex items-center space-x-3 space-x-reverse px-4 py-3 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors"
                                            }}
                                        >
                                            <span class="text-lg">{icon}</span>
                                            <span>{label}</span>
                                        </button>
                                    </li>
                                }
                            })}
                        </ul>
                    </nav>
                </aside>

                // Main Content
                <main class="flex-1 p-6">
                    {match *active_tab {
                        DashboardTab::Overview => render_overview(stats),
                        DashboardTab::Products => render_products(),
                        DashboardTab::Orders => render_orders(),
                        DashboardTab::Users => render_users(),
                        DashboardTab::Vendors => render_vendors(),
                        DashboardTab::Inventory => render_inventory(),
                        DashboardTab::Notifications => render_notifications(),
                        DashboardTab::Settings => render_settings(),
                    }}
                </main>
            </div>
        </div>
    }
}

fn render_overview(stats: &DashboardStats) -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"نمای کلی"}</h2>
                <p class="text-gray-600">{"خلاصه‌ای از وضعیت کلی پلتفرم"}</p>
            </div>

            // Stats Grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <StatsWidget 
                    title="کل محصولات" 
                    value={stats.total_products.to_string()} 
                    icon="📦" 
                    color="text-blue-600" 
                />
                <StatsWidget 
                    title="کل سفارشات" 
                    value={stats.total_orders.to_string()} 
                    icon="🛍️" 
                    color="text-green-600" 
                />
                <StatsWidget 
                    title="کل کاربران" 
                    value={stats.total_users.to_string()} 
                    icon="👥" 
                    color="text-purple-600" 
                />
                <StatsWidget 
                    title="کل فروشندگان" 
                    value={stats.total_vendors.to_string()} 
                    icon="🏪" 
                    color="text-orange-600" 
                />
            </div>

            // Alert Cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-6">
                    <div class="flex items-center space-x-3 space-x-reverse">
                        <div class="text-2xl">{"⏳"}</div>
                        <div>
                            <h3 class="text-lg font-semibold text-yellow-800">{"سفارشات در انتظار"}</h3>
                            <p class="text-2xl font-bold text-yellow-600">{stats.pending_orders}</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-red-50 border border-red-200 rounded-lg p-6">
                    <div class="flex items-center space-x-3 space-x-reverse">
                        <div class="text-2xl">{"📉"}</div>
                        <div>
                            <h3 class="text-lg font-semibold text-red-800">{"کالاهای کم موجود"}</h3>
                            <p class="text-2xl font-bold text-red-600">{stats.low_stock_items}</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-green-50 border border-green-200 rounded-lg p-6">
                    <div class="flex items-center space-x-3 space-x-reverse">
                        <div class="text-2xl">{"💰"}</div>
                        <div>
                            <h3 class="text-lg font-semibold text-green-800">{"درآمد ماهانه"}</h3>
                            <p class="text-xl font-bold text-green-600">{format!("{:.0} تومان", stats.monthly_revenue)}</p>
                        </div>
                    </div>
                </div>
            </div>

            // Recent Activity
            <div class="bg-white rounded-lg shadow-sm p-6">
                <h3 class="text-xl font-semibold text-gray-900 mb-4">{"فعالیت‌های اخیر"}</h3>
                <div class="space-y-4">
                    <div class="flex items-center space-x-3 space-x-reverse p-3 bg-gray-50 rounded-lg">
                        <div class="text-lg">{"🆕"}</div>
                        <div class="flex-1">
                            <p class="text-sm font-medium text-gray-900">{"محصول جدید اضافه شد"}</p>
                            <p class="text-xs text-gray-500">{"5 دقیقه پیش"}</p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-3 space-x-reverse p-3 bg-gray-50 rounded-lg">
                        <div class="text-lg">{"✅"}</div>
                        <div class="flex-1">
                            <p class="text-sm font-medium text-gray-900">{"سفارش جدید تأیید شد"}</p>
                            <p class="text-xs text-gray-500">{"10 دقیقه پیش"}</p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-3 space-x-reverse p-3 bg-gray-50 rounded-lg">
                        <div class="text-lg">{"👤"}</div>
                        <div class="flex-1">
                            <p class="text-sm font-medium text-gray-900">{"کاربر جدید ثبت‌نام کرد"}</p>
                            <p class="text-xs text-gray-500">{"15 دقیقه پیش"}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_products() -> Html {
    html! {
        <ProductManagement />
    }
}

fn render_orders() -> Html {
    html! {
        <OrderManagement />
    }
}

fn render_users() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت کاربران"}</h2>
                <p class="text-gray-600">{"مدیریت کاربران و دسترسی‌ها"}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm p-6">
                <p class="text-gray-600">{"بخش مدیریت کاربران در حال توسعه است."}</p>
            </div>
        </div>
    }
}

fn render_vendors() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت فروشندگان"}</h2>
                <p class="text-gray-600">{"مدیریت فروشندگان و شرکای تجاری"}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm p-6">
                <p class="text-gray-600">{"بخش مدیریت فروشندگان در حال توسعه است."}</p>
            </div>
        </div>
    }
}

fn render_inventory() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت موجودی"}</h2>
                <p class="text-gray-600">{"کنترل و مدیریت موجودی انبار"}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm p-6">
                <p class="text-gray-600">{"بخش مدیریت موجودی در حال توسعه است."}</p>
            </div>
        </div>
    }
}

fn render_notifications() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت اطلاع‌رسانی"}</h2>
                <p class="text-gray-600">{"ارسال و مدیریت اطلاع‌رسانی‌ها"}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm p-6">
                <p class="text-gray-600">{"بخش مدیریت اطلاع‌رسانی در حال توسعه است."}</p>
            </div>
        </div>
    }
}

fn render_settings() -> Html {
    html! {
        <div>
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"تنظیمات سیستم"}</h2>
                <p class="text-gray-600">{"پیکربندی و تنظیمات پلتفرم"}</p>
            </div>
            <div class="bg-white rounded-lg shadow-sm p-6">
                <p class="text-gray-600">{"بخش تنظیمات در حال توسعه است."}</p>
            </div>
        </div>
    }
}
