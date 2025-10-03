use yew::prelude::*;
use models::order::Order;
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq)]
pub struct OrderItemProps {
    pub order: Order,
    #[prop_or_default]
    pub on_view: Option<Callback<Order>>,
    #[prop_or_default]
    pub on_update_status: Option<Callback<(Order, String)>>,
    #[prop_or_default]
    pub show_actions: bool,
}

#[function_component(OrderItem)]
pub fn order_item(props: &OrderItemProps) -> Html {
    let order = &props.order;
    let on_view = props.on_view.clone();
    let on_update_status = props.on_update_status.clone();

    let handle_view = {
        let order = order.clone();
        let on_view = on_view.clone();
        Callback::from(move |_| {
            if let Some(callback) = &on_view {
                callback.emit(order.clone());
            }
        })
    };

    let handle_status_change = {
        let order = order.clone();
        let on_update_status = on_update_status.clone();
        Callback::from(move |e: Event| {
            if let Some(callback) = &on_update_status {
                if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                    let new_status = target.value();
                    callback.emit((order.clone(), new_status));
                }
            }
        })
    };

    let (status_class, status_icon) = match order.status.as_str() {
        "pending" => ("bg-yellow-100 text-yellow-800", "⏳"),
        "processing" => ("bg-blue-100 text-blue-800", "🔄"),
        "shipped" => ("bg-purple-100 text-purple-800", "🚚"),
        "delivered" => ("bg-green-100 text-green-800", "✅"),
        "cancelled" => ("bg-red-100 text-red-800", "❌"),
        _ => ("bg-gray-100 text-gray-800", "❓"),
    };

    let status_text = match order.status.as_str() {
        "pending" => "در انتظار",
        "processing" => "در حال پردازش",
        "shipped" => "ارسال شده",
        "delivered" => "تحویل داده شده",
        "cancelled" => "لغو شده",
        _ => "نامشخص",
    };

    html! {
        <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 border border-gray-200 p-4">
            <div class="flex items-center justify-between mb-4">
                <div class="flex items-center space-x-3 space-x-reverse">
                    <div class="text-2xl">{status_icon}</div>
                    <div>
                        <h3 class="text-lg font-semibold text-gray-900">
                            {"سفارش #"}{order.id.to_string()[..8].to_uppercase()}
                        </h3>
                        <p class="text-sm text-gray-500">
                            {"تاریخ ثبت: "} {order.created_at.format("%Y/%m/%d %H:%M").to_string()}
                        </p>
                    </div>
                </div>
                <span class={format!("px-3 py-1 text-sm font-medium rounded-full {}", status_class)}>
                    {status_text}
                </span>
            </div>

            <div class="grid grid-cols-2 gap-4 mb-4">
                <div>
                    <p class="text-sm text-gray-500">{"مبلغ کل"}</p>
                    <p class="text-xl font-bold text-purple-600">
                        {format!("{:.0} تومان", order.total_amount)}
                    </p>
                </div>
                <div>
                    <p class="text-sm text-gray-500">{"شناسه کاربر"}</p>
                    <p class="text-sm font-mono text-gray-700">
                        {order.user_id.to_string()[..8].to_uppercase()}
                    </p>
                </div>
            </div>

            if order.updated_at != order.created_at {
                <div class="mb-4">
                    <p class="text-xs text-gray-400">
                        {"آخرین بروزرسانی: "} {order.updated_at.format("%Y/%m/%d %H:%M").to_string()}
                    </p>
                </div>
            }

            if props.show_actions {
                <div class="flex space-x-2 space-x-reverse">
                    <button 
                        onclick={handle_view}
                        class="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-sm py-2 px-3 rounded-md transition-colors duration-200"
                    >
                        {"مشاهده جزئیات"}
                    </button>
                    
                    if order.status != "delivered" && order.status != "cancelled" {
                        <select 
                            onchange={handle_status_change}
                            class="flex-1 border border-gray-300 rounded-md text-sm py-2 px-3 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        >
                            <option value="" disabled=true selected={true}>{"تغییر وضعیت"}</option>
                            <option value="pending">{"در انتظار"}</option>
                            <option value="processing">{"در حال پردازش"}</option>
                            <option value="shipped">{"ارسال شده"}</option>
                            <option value="delivered">{"تحویل داده شده"}</option>
                            <option value="cancelled">{"لغو شده"}</option>
                        </select>
                    }
                </div>
            }
        </div>
    }
}
