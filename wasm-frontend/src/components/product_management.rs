use yew::prelude::*;
use models::product::Product;
use crate::components::{ProductCard, PaginationComponent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use web_sys::HtmlInputElement;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductFilter {
    All,
    InStock,
    LowStock,
    OutOfStock,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProductCategory {
    All,
    Electronics,
    Clothing,
    Books,
    Home,
    Sports,
}

#[derive(Properties, PartialEq)]
pub struct ProductManagementProps {
    #[prop_or_default]
    pub products: Vec<Product>,
    #[prop_or_default]
    pub on_product_update: Option<Callback<Product>>,
    #[prop_or_default]
    pub on_product_delete: Option<Callback<Uuid>>,
    #[prop_or_default]
    pub on_product_create: Option<Callback<Product>>,
}

#[function_component(ProductManagement)]
pub fn product_management(props: &ProductManagementProps) -> Html {
    let search_term = use_state(|| String::new());
    let selected_filter = use_state(|| ProductFilter::All);
    let selected_category = use_state(|| ProductCategory::All);
    let current_page = use_state(|| 1u32);
    let show_create_modal = use_state(|| false);
    let show_edit_modal = use_state(|| false);
    let editing_product = use_state(|| None::<Product>);

    // Sample products for demonstration
    let sample_products = vec![
        Product {
            id: Uuid::new_v4(),
            name: "گوشی هوشمند سامسونگ".to_string(),
            description: Some("گوشی هوشمند با کیفیت بالا".to_string()),
            price: 15000000.0,
            stock: 25,
            category: "Electronics".to_string(),
            vendor_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Product {
            id: Uuid::new_v4(),
            name: "لپ‌تاپ ایسوس".to_string(),
            description: Some("لپ‌تاپ گیمینگ قدرتمند".to_string()),
            price: 25000000.0,
            stock: 12,
            category: "Electronics".to_string(),
            vendor_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Product {
            id: Uuid::new_v4(),
            name: "تی‌شرت نخی".to_string(),
            description: Some("تی‌شرت راحت و با کیفیت".to_string()),
            price: 150000.0,
            stock: 50,
            category: "Clothing".to_string(),
            vendor_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    let products = if props.products.is_empty() {
        sample_products
    } else {
        props.products.clone()
    };

    let handle_search = {
        let search_term = search_term.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                search_term.set(input.value());
            }
        })
    };

    let handle_filter_change = {
        let selected_filter = selected_filter.clone();
        Callback::from(move |filter: ProductFilter| {
            selected_filter.set(filter);
        })
    };

    let handle_category_change = {
        let selected_category = selected_category.clone();
        Callback::from(move |category: ProductCategory| {
            selected_category.set(category);
        })
    };

    let handle_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
        })
    };

    let handle_create_product = {
        let show_create_modal = show_create_modal.clone();
        Callback::from(move |_| {
            show_create_modal.set(true);
        })
    };

    let handle_edit_product = {
        let show_edit_modal = show_edit_modal.clone();
        let editing_product = editing_product.clone();
        Callback::from(move |product: Product| {
            editing_product.set(Some(product));
            show_edit_modal.set(true);
        })
    };

    let handle_close_modals = {
        let show_create_modal = show_create_modal.clone();
        let show_edit_modal = show_edit_modal.clone();
        let editing_product = editing_product.clone();
        Callback::from(move |_| {
            show_create_modal.set(false);
            show_edit_modal.set(false);
            editing_product.set(None);
        })
    };

    // Filter products based on search and filters
    let filtered_products: Vec<Product> = products
        .into_iter()
        .filter(|product| {
            let matches_search = search_term.is_empty() || 
                product.name.contains(&*search_term) ||
                product.description.as_ref().map_or(false, |desc| desc.contains(&*search_term));
            
            let matches_filter = match *selected_filter {
                ProductFilter::All => true,
                ProductFilter::InStock => product.stock > 10,
                ProductFilter::LowStock => product.stock > 0 && product.stock <= 10,
                ProductFilter::OutOfStock => product.stock == 0,
            };

            let matches_category = match *selected_category {
                ProductCategory::All => true,
                ProductCategory::Electronics => product.category == "Electronics",
                ProductCategory::Clothing => product.category == "Clothing",
                ProductCategory::Books => product.category == "Books",
                ProductCategory::Home => product.category == "Home",
                ProductCategory::Sports => product.category == "Sports",
            };

            matches_search && matches_filter && matches_category
        })
        .collect();

    let total_products = filtered_products.len() as u32;
    let products_per_page = 12u32;
    let total_pages = (total_products + products_per_page - 1) / products_per_page;
    let start_index = ((*current_page - 1) * products_per_page) as usize;
    let end_index = std::cmp::min(start_index + products_per_page as usize, filtered_products.len());
    let page_products = &filtered_products[start_index..end_index];

    html! {
        <div>
            // Header
            <div class="flex items-center justify-between mb-8">
                <div>
                    <h2 class="text-3xl font-bold text-gray-900 mb-2">{"مدیریت محصولات"}</h2>
                    <p class="text-gray-600">{format!("مجموع {} محصول", total_products)}</p>
                </div>
                <button 
                    onclick={handle_create_product}
                    class="bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-lg font-medium transition-colors flex items-center space-x-2 space-x-reverse"
                >
                    <span>{"+"}</span>
                    <span>{"افزودن محصول"}</span>
                </button>
            </div>

            // Search and Filters
            <div class="bg-white rounded-lg shadow-sm p-6 mb-6">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <div class="md:col-span-2">
                        <input 
                            type="text" 
                            placeholder="جستجوی محصولات..." 
                            value={(*search_term).clone()}
                            oninput={handle_search}
                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                        />
                    </div>
                    
                    <div>
                        <select 
                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                            onchange={Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    let filter = match select.value().as_str() {
                                        "in_stock" => ProductFilter::InStock,
                                        "low_stock" => ProductFilter::LowStock,
                                        "out_of_stock" => ProductFilter::OutOfStock,
                                        _ => ProductFilter::All,
                                    };
                                    handle_filter_change.emit(filter);
                                }
                            })}
                        >
                            <option value="">{"وضعیت موجودی"}</option>
                            <option value="in_stock">{"موجود"}</option>
                            <option value="low_stock">{"کم موجود"}</option>
                            <option value="out_of_stock">{"ناموجود"}</option>
                        </select>
                    </div>
                    
                    <div>
                        <select 
                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                            onchange={Callback::from(move |e: Event| {
                                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                    let category = match select.value().as_str() {
                                        "electronics" => ProductCategory::Electronics,
                                        "clothing" => ProductCategory::Clothing,
                                        "books" => ProductCategory::Books,
                                        "home" => ProductCategory::Home,
                                        "sports" => ProductCategory::Sports,
                                        _ => ProductCategory::All,
                                    };
                                    handle_category_change.emit(category);
                                }
                            })}
                        >
                            <option value="">{"همه دسته‌ها"}</option>
                            <option value="electronics">{"الکترونیک"}</option>
                            <option value="clothing">{"پوشاک"}</option>
                            <option value="books">{"کتاب"}</option>
                            <option value="home">{"خانه و آشپزخانه"}</option>
                            <option value="sports">{"ورزش"}</option>
                        </select>
                    </div>
                </div>
            </div>

            // Products Grid
            if page_products.is_empty() {
                <div class="bg-white rounded-lg shadow-sm p-12 text-center">
                    <div class="text-6xl mb-4">{"📦"}</div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-2">{"محصولی یافت نشد"}</h3>
                    <p class="text-gray-600">{"با فیلترهای مختلف جستجو کنید یا محصول جدید اضافه کنید."}</p>
                </div>
            } else {
                <>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6 mb-6">
                        {for page_products.iter().map(|product| {
                            let product_clone = product.clone();
                            let handle_edit = {
                                let handle_edit_product = handle_edit_product.clone();
                                Callback::from(move |_| {
                                    handle_edit_product.emit(product_clone.clone());
                                })
                            };
                            
                            html! {
                                <div class="bg-white rounded-lg shadow-sm overflow-hidden hover:shadow-md transition-shadow">
                                    <div class="p-4">
                                        <div class="text-4xl text-center mb-4">
                                            {match product.category.as_str() {
                                                "Electronics" => "📱",
                                                "Clothing" => "👕",
                                                "Books" => "📚",
                                                "Home" => "🏠",
                                                "Sports" => "⚽",
                                                _ => "📦",
                                            }}
                                        </div>
                                        <h3 class="font-semibold text-gray-900 mb-2 text-center">{&product.name}</h3>
                                        <p class="text-sm text-gray-600 mb-3 text-center line-clamp-2">
                                            {product.description.as_ref().unwrap_or(&"توضیحات موجود نیست".to_string())}
                                        </p>
                                        <div class="text-center mb-3">
                                            <p class="text-lg font-bold text-purple-600">{format!("{:.0} تومان", product.price)}</p>
                                            <p class={format!("text-sm {}", 
                                                if product.stock > 10 { "text-green-600" }
                                                else if product.stock > 0 { "text-yellow-600" }
                                                else { "text-red-600" }
                                            )}>
                                                {format!("موجودی: {}", product.stock)}
                                            </p>
                                        </div>
                                        <div class="flex space-x-2 space-x-reverse">
                                            <button 
                                                onclick={handle_edit}
                                                class="flex-1 bg-blue-600 hover:bg-blue-700 text-white py-2 px-3 rounded text-sm transition-colors"
                                            >
                                                {"ویرایش"}
                                            </button>
                                            <button class="flex-1 bg-red-600 hover:bg-red-700 text-white py-2 px-3 rounded text-sm transition-colors">
                                                {"حذف"}
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
                                total_items={total_products}
                                limit={products_per_page}
                                on_page_change={handle_page_change}
                            />
                        </div>
                    }
                </>
            }

            // Create Product Modal
            if *show_create_modal {
                <ProductModal 
                    product={None}
                    on_close={handle_close_modals.clone()}
                    on_save={Callback::from(|_| {})}
                />
            }

            // Edit Product Modal
            if *show_edit_modal {
                <ProductModal 
                    product={(*editing_product).clone()}
                    on_close={handle_close_modals.clone()}
                    on_save={Callback::from(|_| {})}
                />
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProductModalProps {
    pub product: Option<Product>,
    pub on_close: Callback<()>,
    pub on_save: Callback<Product>,
}

#[function_component(ProductModal)]
pub fn product_modal(props: &ProductModalProps) -> Html {
    let name = use_state(|| props.product.as_ref().map(|p| p.name.clone()).unwrap_or_default());
    let description = use_state(|| props.product.as_ref().map(|p| p.description.clone()).unwrap_or_default());
    let price = use_state(|| props.product.as_ref().map(|p| p.price.to_string()).unwrap_or_default());
    let stock = use_state(|| props.product.as_ref().map(|p| p.stock.to_string()).unwrap_or_default());
    let category = use_state(|| props.product.as_ref().map(|p| p.category.clone()).unwrap_or_default());

    let is_edit = props.product.is_some();
    let title = if is_edit { "ویرایش محصول" } else { "افزودن محصول جدید" };

    html! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full mx-4 max-h-screen overflow-y-auto">
                <div class="p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h3 class="text-xl font-semibold text-gray-900">{title}</h3>
                        <button 
                            onclick={props.on_close.reform(|_| ())}
                            class="text-gray-400 hover:text-gray-600 transition-colors"
                        >
                            <span class="text-2xl">{"×"}</span>
                        </button>
                    </div>

                    <form class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">{"نام محصول"}</label>
                            <input 
                                type="text" 
                                value={(*name).clone()}
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                                placeholder="نام محصول را وارد کنید"
                            />
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">{"توضیحات"}</label>
                            <textarea 
                                value={(*description).clone()}
                                rows="3"
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                                placeholder="توضیحات محصول را وارد کنید"
                            ></textarea>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">{"قیمت (تومان)"}</label>
                                <input 
                                    type="number" 
                                    value={(*price).clone()}
                                    class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                                    placeholder="0"
                                />
                            </div>

                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-2">{"موجودی"}</label>
                                <input 
                                    type="number" 
                                    value={(*stock).clone()}
                                    class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                                    placeholder="0"
                                />
                            </div>
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">{"دسته‌بندی"}</label>
                            <select 
                                value={(*category).clone()}
                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                            >
                                <option value="">{"انتخاب دسته‌بندی"}</option>
                                <option value="Electronics">{"الکترونیک"}</option>
                                <option value="Clothing">{"پوشاک"}</option>
                                <option value="Books">{"کتاب"}</option>
                                <option value="Home">{"خانه و آشپزخانه"}</option>
                                <option value="Sports">{"ورزش"}</option>
                            </select>
                        </div>

                        <div class="flex space-x-4 space-x-reverse pt-6">
                            <button 
                                type="button"
                                onclick={props.on_close.reform(|_| ())}
                                class="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
                            >
                                {"انصراف"}
                            </button>
                            <button 
                                type="submit"
                                class="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors"
                            >
                                {if is_edit { "ذخیره تغییرات" } else { "افزودن محصول" }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
