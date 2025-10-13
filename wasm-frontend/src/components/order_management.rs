use yew::prelude::*;
use models::order::Order;
use crate::components::{OrderItem, PaginationComponent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use web_sys::HtmlInputElement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    All,
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn to_string(&self) -> String {
        match self {
            OrderStatus::All => "همه".to_string(),
            OrderStatus::Pending => "در انتظار".to_string(),
            OrderStatus::Processing => "در حال پردازش".to_string(),
            OrderStatus::Shipped => "ارسال شده".to_string(),
            OrderStatus::Delivered => "تحویل داده شده".to_string(),
            OrderStatus::Cancelled => "لغو شده".to_string(),
        }
    }

    pub fn to_badge_class(&self) -> &'static str {
        match self {
            OrderStatus::Pending => "bg-yellow-100 text-yellow-800",
            OrderStatus::Processing => "bg-blue-100 text-blue-800",
            OrderStatus::Shipped => "bg-purple-100 text-purple-800",
            OrderStatus::Delivered => "bg-green-100 text-green-800",
            OrderStatus::Cancelled => "bg-red-100 text-red-800",
            OrderStatus::All => "bg-gray-100 text-gray-800",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct OrderManagementProps {
    #[prop_or_default]
    pub orders: Vec<Order>,
    #[prop_or_default]
    pub on_order_update: Option<Callback<(Order, String)>>,
    #[prop_or_default]
    pub on_order_delete: Option<Callback<Uuid>>,
}

#[function_component(OrderManagement)]
pub fn order_management(props: &OrderManagementProps) -> Html {
    let search_term = use_state(|| String::new());
    let selected_status = use_state(|| OrderStatus::All);
    let current_page = use_state(|| 1u32);
    let show_order_details = use_state(|| false);
    let selected_order = use_state(|| None::<Order>);

    // Sample orders for demonstration
    let sample_orders = vec![
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            total_amount: 2500000.0,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            total_amount: 1200000.0,
            status: "processing".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            total_amount: 850000.0,
            status: "shipped".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Order {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            total_amount: 3200000.0,
            status: "delivered".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    let orders = if props.orders.is_empty() {
        sample_orders.clone()
    } else {
        props.orders.clone()
    };

    let handle_search = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                search_term.set(input.value());
            }
        })
    };

    let handle_status_change = {
        let selected_status = selected_status.clone();
        Callback::from(move |status: OrderStatus| {
            selected_status.set(status);
        })
    };

    let handle_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
        })
    };

    let handle_view_order = {
        let show_order_details = show_order_details.clone();
        let selected_order = selected_order.clone();
        Callback::from(move |order: Order| {
            selected_order.set(Some(order));
            show_order_details.set(true);
        })
    };

    let handle_close_details = {
        let show_order_details = show_order_details.clone();
        let selected_order = selected_order.clone();
        Callback::from(move |_| {
            show_order_details.set(false);
            selected_order.set(None);
        })
    };

    // Calculate status counts
    let status_counts = {
        let mut counts = std::collections::HashMap::new();
        for order in &orders {
            *counts.entry(order.status.clone()).or_insert(0) += 1;
        }
        counts
    };

    // Filter orders based on search and status
    let filtered_orders: Vec<Order> = orders
        .into_iter()
        .filter(|order| {
            let matches_search = search_term.is_empty() || 
                order.id.to_string().contains(&*search_term) ||
                order.total_amount.to_string().contains(&*search_term);
            
            let matches_status = match *selected_status {
                OrderStatus::All => true,
                OrderStatus::Pending => order.status == "pending",
                OrderStatus::Processing => order.status == "processing",
                OrderStatus::Shipped => order.status == "shipped",
                OrderStatus::Delivered => order.status == "delivered",
                OrderStatus::Cancelled => order.status == "cancelled",
            };

            matches_search && matches_status
        })
        .collect();

    let total_orders = filtered_orders.len() as u32;
    let orders_per_page = 10u32;
    let total_pages = (total_orders + orders_per_page - 1) / orders_per_page;
    let start_index = ((*current_page - 1) * orders_per_page) as usize;
    let end_index = std::cmp::min(start_index + orders_per_page as usize, filtered_orders.len());
    let page_orders = &filtered_orders[start_index..end_index];

    html! {
        <div>
            // Header
            <div class="mb-8">
                <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت سفارشات"}</h2>
                <p class="text-gray-600">{format!("مجموع {} سفارش", total_orders)}</p>
            </div>

            // Status Tabs
            <div class="bg-white rounded-lg shadow-sm mb-6">
                <div class="border-b border-gray-200">
                    <nav class="flex space-x-8 space-x-reverse px-6">
                        {for [
                            OrderStatus::All,
                            OrderStatus::Pending,
                            OrderStatus::Processing,
                            OrderStatus::Shipped,
                            OrderStatus::Delivered,
                            OrderStatus::Cancelled,
                        ].iter().map(|status| {
                            let is_active = *selected_status == *status;
                            let status_clone = status.clone();
                            let count = match status {
                                OrderStatus::All => sample_orders.len(),
                                OrderStatus::Pending => *status_counts.get("pending").unwrap_or(&0),
                                OrderStatus::Processing => *status_counts.get("processing").unwrap_or(&0),
                                OrderStatus::Shipped => *status_counts.get("shipped").unwrap_or(&0),
                                OrderStatus::Delivered => *status_counts.get("delivered").unwrap_or(&0),
                                OrderStatus::Cancelled => *status_counts.get("cancelled").unwrap_or(&0),
                            };
                            
                            let handle_click = {
                                let handle_status_change = handle_status_change.clone();
                                Callback::from(move |_| {
                                    handle_status_change.emit(status_clone.clone());
                                })
                            };
                            
                            html! {
                                <button
                                    onclick={handle_click}
                                    class={if is_active {
                                        "py-4 px-1 border-b-2 border-purple-500 text-purple-600 font-medium flex items-center space-x-2 space-x-reverse"
                                    } else {
                                        "py-4 px-1 border-b-2 border-transparent text-gray-500 hover:text-gray-700 flex items-center space-x-2 space-x-reverse"
                                    }}
                                >
                                    <span>{status.to_string()}</span>
                                    <span class="bg-gray-100 text-gray-600 text-xs px-2 py-1 rounded-full">
                                        {count}
                                    </span>
                                </button>
                            }
                        })}
                    </nav>
                </div>
            </div>

            // Search and Filters
            <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
                <div class="flex flex-col md:flex-row gap-4">
                    <div class="flex-1">
                        <input 
                            type="text" 
                            placeholder="جستجو بر اساس شماره سفارش یا مبلغ..." 
                            value={(*search_term).clone()}
                            oninput={handle_search}
                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        />
                    </div>
                    <div class="flex space-x-2 space-x-reverse">
                        <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors">
                            {"خروجی Excel"}
                        </button>
                        <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                            {"گزارش"}
                        </button>
                    </div>
                </div>
            </div>

            // Orders List
            if page_orders.is_empty() {
                <div class="bg-white rounded-lg shadow-sm p-12 text-center">
                    <div class="text-6xl mb-4">{"📋"}</div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-2">{"سفارشی یافت نشد"}</h3>
                    <p class="text-gray-600">{"با فیلترهای مختلف جستجو کنید."}</p>
                </div>
            } else {
                <>
                    <div class="space-y-4 mb-6">
                        {for page_orders.iter().map(|order| {
                            let order_clone = order.clone();
                            let handle_view = {
                                let handle_view_order = handle_view_order.clone();
                                Callback::from(move |_| {
                                    handle_view_order.emit(order_clone.clone());
                                })
                            };
                            
                            let status_from_string = match order.status.as_str() {
                                "pending" => OrderStatus::Pending,
                                "processing" => OrderStatus::Processing,
                                "shipped" => OrderStatus::Shipped,
                                "delivered" => OrderStatus::Delivered,
                                "cancelled" => OrderStatus::Cancelled,
                                _ => OrderStatus::All,
                            };
                            
                            html! {
                                <div class="bg-white rounded-lg shadow-sm p-6 hover:shadow-md transition-shadow">
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center space-x-4 space-x-reverse">
                                            <div class="text-3xl">{"📦"}</div>
                                            <div>
                                                <h3 class="font-semibold text-gray-900">
                                                    {format!("سفارش #{}", &order.id.to_string()[..8])}
                                                </h3>
                                                <p class="text-sm text-gray-500">
                                                    {format!("تاریخ: {}", order.created_at.format("%Y/%m/%d").to_string())}
                                                </p>
                                                <p class="text-sm text-gray-500">
                                                    {format!("کاربر: {}", &order.user_id.to_string()[..8])}
                                                </p>
                                            </div>
                                        </div>
                                        <div class="text-left">
                                            <p class="text-xl font-bold text-purple-600 mb-2">
                                                {format!("{:.0} تومان", order.total_amount)}
                                            </p>
                                            <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}", 
                                                status_from_string.to_badge_class())}>
                                                {status_from_string.to_string()}
                                            </span>
                                        </div>
                                        <div class="flex space-x-2 space-x-reverse">
                                            <button 
                                                onclick={handle_view}
                                                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-sm"
                                            >
                                                {"مشاهده"}
                                            </button>
                                            <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors text-sm">
                                                {"ویرایش"}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            }
                        })}
                    </div>

                    // Pagination
                    if total_pages > 1 {
                        <div class="bg-white rounded-lg shadow-sm">
                            <PaginationComponent 
                                current_page={*current_page}
                                total_pages={total_pages}
                                total_items={total_orders}
                                limit={orders_per_page}
                                on_page_change={handle_page_change}
                            />
                        </div>
                    }
                </>
            }

            // Order Details Modal
            if *show_order_details {
                if let Some(order) = &*selected_order {
                    <OrderDetailsModal 
                        order={order.clone()}
                        on_close={handle_close_details}
                    />
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct OrderDetailsModalProps {
    pub order: Order,
    pub on_close: Callback<()>,
}

#[function_component(OrderDetailsModal)]
pub fn order_details_modal(props: &OrderDetailsModalProps) -> Html {
    let order = &props.order;
    
    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div class="bg-white rounded-lg shadow-xl max-w-4xl w-full mx-4 max-h-screen overflow-y-auto">
                <div class="p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h3 class="text-xl font-semibold text-gray-900">
                            {format!("جزئیات سفارش #{}", &order.id.to_string()[..8])}
                        </h3>
                        <button 
                            onclick={props.on_close.reform(|_| ())}
                            class="text-gray-400 hover:text-gray-600 transition-colors"
                        >
                            <span class="text-2xl">{"×"}</span>
                        </button>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        // Order Information
                        <div class="bg-gray-50 rounded-lg p-4">
                            <h4 class="font-semibold text-gray-900 mb-4">{"اطلاعات سفارش"}</h4>
                            <div class="space-y-3">
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"شماره سفارش:"}</span>
                                    <span class="font-medium">{"#"}{&order.id.to_string()[..8]}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"تاریخ ثبت:"}</span>
                                    <span class="font-medium">{order.created_at.format("%Y/%m/%d %H:%M").to_string()}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"وضعیت:"}</span>
                                    <span class="font-medium">{&order.status}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"مبلغ کل:"}</span>
                                    <span class="font-bold text-purple-600">{format!("{:.0} تومان", order.total_amount)}</span>
                                </div>
                            </div>
                        </div>

                        // Customer Information
                        <div class="bg-gray-50 rounded-lg p-4">
                            <h4 class="font-semibold text-gray-900 mb-4">{"اطلاعات مشتری"}</h4>
                            <div class="space-y-3">
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"شناسه مشتری:"}</span>
                                    <span class="font-medium">{"#"}{&order.user_id.to_string()[..8]}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"نام:"}</span>
                                    <span class="font-medium">{"احمد محمدی"}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"تلفن:"}</span>
                                    <span class="font-medium">{"09123456789"}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-gray-600">{"ایمیل:"}</span>
                                    <span class="font-medium">{"ahmad@example.com"}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Order Items
                    <div class="mt-6">
                        <h4 class="font-semibold text-gray-900 mb-4">{"اقلام سفارش"}</h4>
                        <div class="bg-gray-50 rounded-lg p-4">
                            <div class="space-y-3">
                                <div class="flex items-center justify-between py-3 border-b border-gray-200">
                                    <div class="flex items-center space-x-3 space-x-reverse">
                                        <div class="text-2xl">{"📱"}</div>
                                        <div>
                                            <h5 class="font-medium">{"گوشی هوشمند سامسونگ"}</h5>
                                            <p class="text-sm text-gray-600">{"تعداد: 1"}</p>
                                        </div>
                                    </div>
                                    <span class="font-medium">{"15,000,000 تومان"}</span>
                                </div>
                                <div class="flex items-center justify-between py-3 border-b border-gray-200">
                                    <div class="flex items-center space-x-3 space-x-reverse">
                                        <div class="text-2xl">{"🎧"}</div>
                                        <div>
                                            <h5 class="font-medium">{"هدفون بلوتوثی"}</h5>
                                            <p class="text-sm text-gray-600">{"تعداد: 2"}</p>
                                        </div>
                                    </div>
                                    <span class="font-medium">{"1,500,000 تومان"}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Action Buttons
                    <div class="flex space-x-4 space-x-reverse pt-6 border-t border-gray-200 mt-6">
                        <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors">
                            {"تأیید سفارش"}
                        </button>
                        <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                            {"ارسال سفارش"}
                        </button>
                        <button class="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors">
                            {"لغو سفارش"}
                        </button>
                        <button class="px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors">
                            {"چاپ فاکتور"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
