use yew::prelude::*;
use crate::components::{Button, Card, Input};
use crate::services::api::ApiService;

#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub weight: f64,
    pub purity: String,
    pub stock: i32,
    pub image_url: Option<String>,
}

#[function_component(Products)]
pub fn products() -> Html {
    let products = use_state(|| vec![]);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Load products from WASM backend on component mount
    use_effect_with((), {
        let products = products.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let api_service = ApiService::new("http://localhost:8080/api");
                match api_service.fetch_products().await {
                    Ok(_result) => {
                        // For demo purposes, use static data
                        // In a real implementation, we would parse the result from WASM backend
                        let demo_products = vec![
                            Product {
                                id: "1".to_string(),
                                name: "گردآفرید کلاسیک".to_string(),
                                description: "سکه نقره دولایه با طراحی سنتی".to_string(),
                                price: 2500000.0,
                                weight: 15.0,
                                purity: "۹۹۰".to_string(),
                                stock: 25,
                                image_url: None,
                            },
                            Product {
                                id: "2".to_string(),
                                name: "گردآفرید مدرن".to_string(),
                                description: "سکه نقره با طراحی معاصر".to_string(),
                                price: 2800000.0,
                                weight: 18.0,
                                purity: "۹۹۰".to_string(),
                                stock: 15,
                                image_url: None,
                            },
                            Product {
                                id: "3".to_string(),
                                name: "گردآفرید لوکس".to_string(),
                                description: "سکه نقره پرمیوم با جعبه چوبی".to_string(),
                                price: 3200000.0,
                                weight: 20.0,
                                purity: "۹۹۹".to_string(),
                                stock: 8,
                                image_url: None,
                            },
                        ];
                        products.set(demo_products);
                        loading.set(false);
                    },
                    Err(err_msg) => {
                        error.set(Some(err_msg.to_string()));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        }
    });

    let search_term = use_state(|| String::new());
    let show_add_modal = use_state(|| false);

    let on_search_change = {
        let search_term = search_term.clone();
        Callback::from(move |value: String| {
            search_term.set(value);
        })
    };

    let on_add_product = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| {
            show_add_modal.set(true);
        })
    };

    let on_close_modal = {
        let show_add_modal = show_add_modal.clone();
        Callback::from(move |_| {
            show_add_modal.set(false);
        })
    };

    let filtered_products: Vec<Product> = products
        .iter()
        .filter(|product| {
            if search_term.is_empty() {
                true
            } else {
                product.name.contains(&**search_term) || 
                product.description.contains(&**search_term)
            }
        })
        .cloned()
        .collect();

    html! {
        <div class="products-page">
            <div class="products-header">
                <h1>{"مدیریت محصولات"}</h1>
                <p>{"مدیریت محصولات نقره‌ای پلتفرم پما"}</p>
            </div>

            if let Some(err) = error.as_ref() {
                <div class="error-message">
                    <h3>{"خطا در بارگذاری محصولات"}</h3>
                    <p>{err}</p>
                </div>
            }

            if *loading {
                <div class="loading-state">
                    <div class="loading-spinner"></div>
                    <p>{"در حال بارگذاری محصولات..."}</p>
                </div>
            } else {

            <div class="products-toolbar">
                <div class="search-section">
                    <Input
                        placeholder="جستجو در محصولات..."
                        value={(*search_term).clone()}
                        onchange={on_search_change}
                    />
                </div>
                
                <div class="actions-section">
                    <Button onclick={on_add_product} variant="primary">
                        {"+ افزودن محصول جدید"}
                    </Button>
                </div>
            </div>

            <div class="products-grid">
                { for filtered_products.iter().map(|product| {

                    html! {
                        <Card class="product-card" key={product.id.clone()}>
                            <div class="product-image">
                                if let Some(image_url) = &product.image_url {
                                    <img src={image_url.clone()} alt={product.name.clone()} />
                                } else {
                                    <div class="product-placeholder">{"🪙"}</div>
                                }
                            </div>
                            
                            <div class="product-info">
                                <h3 class="product-name">{&product.name}</h3>
                                <p class="product-description">{&product.description}</p>
                                
                                <div class="product-details">
                                    <div class="detail-item">
                                        <span class="detail-label">{"قیمت:"}</span>
                                        <span class="detail-value">{format!("{} تومان", product.price as i64)}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">{"وزن:"}</span>
                                        <span class="detail-value">{format!("{} گرم", product.weight)}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">{"خلوص:"}</span>
                                        <span class="detail-value">{&product.purity}</span>
                                    </div>
                                    <div class="detail-item">
                                        <span class="detail-label">{"موجودی:"}</span>
                                        <span class={format!("detail-value {}", 
                                            if product.stock > 10 { "stock-good" } 
                                            else if product.stock > 0 { "stock-low" } 
                                            else { "stock-out" }
                                        )}>
                                            {product.stock}
                                        </span>
                                    </div>
                                </div>
                                
                                <div class="product-actions">
                                    <Button variant="secondary" size="small">
                                        {"ویرایش"}
                                    </Button>
                                    <Button variant="danger" size="small">
                                        {"حذف"}
                                    </Button>
                                </div>
                            </div>
                        </Card>
                    }
                })}
            </div>

            if filtered_products.is_empty() {
                <div class="empty-state">
                    <h3>{"محصولی یافت نشد"}</h3>
                    <p>{"محصولی با این مشخصات در سیستم موجود نیست"}</p>
                </div>
            }

            if *show_add_modal {
                <div class="modal-overlay" onclick={on_close_modal.clone()}>
                    <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                        <Card title="افزودن محصول جدید" class="add-product-modal">
                            <form>
                                <Input
                                    label="نام محصول"
                                    placeholder="نام محصول را وارد کنید"
                                    required=true
                                />
                                
                                <Input
                                    label="توضیحات"
                                    placeholder="توضیحات محصول"
                                    required=true
                                />
                                
                                <div class="form-row">
                                    <Input
                                        label="قیمت (تومان)"
                                        input_type="number"
                                        placeholder="0"
                                        required=true
                                    />
                                    
                                    <Input
                                        label="وزن (گرم)"
                                        input_type="number"
                                        placeholder="0"
                                        required=true
                                    />
                                </div>
                                
                                <div class="form-row">
                                    <Input
                                        label="خلوص"
                                        placeholder="۹۹۰"
                                        required=true
                                    />
                                    
                                    <Input
                                        label="موجودی اولیه"
                                        input_type="number"
                                        placeholder="0"
                                        required=true
                                    />
                                </div>
                                
                                <div class="modal-actions">
                                    <Button variant="secondary" onclick={on_close_modal}>
                                        {"انصراف"}
                                    </Button>
                                    <Button variant="primary">
                                        {"ذخیره محصول"}
                                    </Button>
                                </div>
                            </form>
                        </Card>
                    </div>
                </div>
            }
            } // Close loading condition
        </div>
    }
}

