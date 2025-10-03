use yew::prelude::*;
use crate::components::{button::Button, card::Card, input::Input};
use crate::services::api::ApiService;

#[derive(Clone, PartialEq)]
pub struct Order {
    pub id: String,
    pub customer_name: String,
    pub product_name: String,
    pub quantity: i32,
    pub total_price: f64,
    pub status: String,
    pub created_at: String,
}

#[function_component(Orders)]
pub fn orders() -> Html {
    let orders = use_state(|| vec![]);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load orders from WASM backend on component mount
    use_effect_with((), {
        let orders = orders.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ApiService::fetch_orders().await {
                    Ok(_result) => {
                        // For demo purposes, use static data
                        let demo_orders = vec![
                            Order {
                                id: "ORD-001".to_string(),
                                customer_name: "احمد رضایی".to_string(),
                                product_name: "گردآفرید کلاسیک".to_string(),
                                quantity: 2,
                                total_price: 5000000.0,
                                status: "در انتظار پردازش".to_string(),
                                created_at: "۱۴۰۳/۰۸/۱۵".to_string(),
                            },
                            Order {
                                id: "ORD-002".to_string(),
                                customer_name: "فاطمه احمدی".to_string(),
                                product_name: "گردآفرید مدرن".to_string(),
                                quantity: 1,
                                total_price: 2800000.0,
                                status: "تکمیل شده".to_string(),
                                created_at: "۱۴۰۳/۰۸/۱۴".to_string(),
                            },
                            Order {
                                id: "ORD-003".to_string(),
                                customer_name: "محمد کریمی".to_string(),
                                product_name: "گردآفرید لوکس".to_string(),
                                quantity: 1,
                                total_price: 3200000.0,
                                status: "در حال ارسال".to_string(),
                                created_at: "۱۴۰۳/۰۸/۱۳".to_string(),
                            },
                        ];
                        orders.set(demo_orders);
                        loading.set(false);
                    },
                    Err(err_msg) => {
                        error.set(Some(err_msg));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        }
    });

    let search_term = use_state(|| String::new());
    let status_filter = use_state(|| "همه".to_string());

    let on_search_change = {
        let search_term = search_term.clone();
        Callback::from(move |value: String| {
            search_term.set(value);
        })
    };

    let filtered_orders: Vec<Order> = orders
        .iter()
        .filter(|order| {
            let search_match = if search_term.is_empty() {
                true
            } else {
                order.id.contains(&**search_term) || 
                order.customer_name.contains(&**search_term) ||
                order.product_name.contains(&**search_term)
            };

            let status_match = if *status_filter == "همه" {
                true
            } else {
                order.status == *status_filter
            };

            search_match && status_match
        })
        .cloned()
        .collect();

    html! {
        <div class="orders-page">
            <div class="orders-header">
                <h1>{"مدیریت سفارشات"}</h1>
                <p>{"پیگیری و مدیریت سفارشات مشتریان"}</p>
            </div>

            <div class="orders-toolbar">
                <div class="search-section">
                    <Input
                        placeholder="جستجو در سفارشات..."
                        value={(*search_term).clone()}
                        onchange={on_search_change}
                    />
                </div>
                
                <div class="filter-section">
                    <select class="status-filter">
                        <option value="همه">{"همه وضعیت‌ها"}</option>
                        <option value="در انتظار پردازش">{"در انتظار پردازش"}</option>
                        <option value="در حال ارسال">{"در حال ارسال"}</option>
                        <option value="تکمیل شده">{"تکمیل شده"}</option>
                        <option value="لغو شده">{"لغو شده"}</option>
                    </select>
                </div>
            </div>

            <div class="orders-table">
                <Card class="table-card">
                    <table class="data-table">
                        <thead>
                            <tr>
                                <th>{"شماره سفارش"}</th>
                                <th>{"نام مشتری"}</th>
                                <th>{"محصول"}</th>
                                <th>{"تعداد"}</th>
                                <th>{"مبلغ کل"}</th>
                                <th>{"وضعیت"}</th>
                                <th>{"تاریخ"}</th>
                                <th>{"عملیات"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for filtered_orders.iter().map(|order| {
                                let status_class = match order.status.as_str() {
                                    "تکمیل شده" => "status-completed",
                                    "در حال ارسال" => "status-shipping",
                                    "در انتظار پردازش" => "status-pending",
                                    "لغو شده" => "status-cancelled",
                                    _ => "status-unknown",
                                };

                                html! {
                                    <tr key={order.id.clone()}>
                                        <td class="order-id">{&order.id}</td>
                                        <td>{&order.customer_name}</td>
                                        <td>{&order.product_name}</td>
                                        <td>{order.quantity}</td>
                                        <td>{format!("{:,} تومان", order.total_price as i64)}</td>
                                        <td>
                                            <span class={format!("status-badge {}", status_class)}>
                                                {&order.status}
                                            </span>
                                        </td>
                                        <td>{&order.created_at}</td>
                                        <td class="actions-cell">
                                            <Button variant="secondary" size="small">
                                                {"مشاهده"}
                                            </Button>
                                            <Button variant="primary" size="small">
                                                {"ویرایش"}
                                            </Button>
                                        </td>
                                    </tr>
                                }
                            })}
                        </tbody>
                    </table>
                </Card>
            </div>

            if filtered_orders.is_empty() {
                <div class="empty-state">
                    <h3>{"سفارشی یافت نشد"}</h3>
                    <p>{"سفارشی با این مشخصات در سیستم موجود نیست"}</p>
                </div>
            }

            <div class="orders-summary">
                <div class="summary-cards">
                    <Card title="کل سفارشات امروز" class="summary-card">
                        <div class="summary-value">{"۱۲"}</div>
                    </Card>
                    
                    <Card title="فروش امروز" class="summary-card">
                        <div class="summary-value">{"۱۵,۲۰۰,۰۰۰ تومان"}</div>
                    </Card>
                    
                    <Card title="سفارشات در انتظار" class="summary-card">
                        <div class="summary-value">{"۵"}</div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
