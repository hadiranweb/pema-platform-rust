use yew::prelude::*;
use models::product::Product;

#[derive(Properties, PartialEq)]
pub struct ProductCardProps {
    pub product: Product,
    #[prop_or_default]
    pub on_edit: Option<Callback<Product>>,
    #[prop_or_default]
    pub on_delete: Option<Callback<Product>>,
    #[prop_or_default]
    pub show_actions: bool,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let product = &props.product;
    let on_edit = props.on_edit.clone();
    let on_delete = props.on_delete.clone();

    let handle_edit = {
        let product = product.clone();
        let on_edit = on_edit.clone();
        Callback::from(move |_| {
            if let Some(callback) = &on_edit {
                callback.emit(product.clone());
            }
        })
    };

    let handle_delete = {
        let product = product.clone();
        let on_delete = on_delete.clone();
        Callback::from(move |_| {
            if let Some(callback) = &on_delete {
                callback.emit(product.clone());
            }
        })
    };

    let stock_status_class = if product.stock > 10 {
        "text-green-600 bg-green-100"
    } else if product.stock > 0 {
        "text-yellow-600 bg-yellow-100"
    } else {
        "text-red-600 bg-red-100"
    };

    let stock_status_text = if product.stock > 10 {
        "موجود"
    } else if product.stock > 0 {
        "کم موجود"
    } else {
        "ناموجود"
    };

    html! {
        <div class="bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 overflow-hidden border border-gray-200">
            // Product Image Placeholder
            <div class="h-48 bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center">
                <span class="text-4xl text-gray-400">{"📦"}</span>
            </div>
            
            // Product Content
            <div class="p-4">
                <div class="flex justify-between items-start mb-2">
                    <h3 class="text-lg font-semibold text-gray-900 truncate flex-1 ml-2">
                        {&product.name}
                    </h3>
                    <span class={format!("px-2 py-1 text-xs font-medium rounded-full {}", stock_status_class)}>
                        {stock_status_text}
                    </span>
                </div>
                
                if let Some(description) = &product.description {
                    <p class="text-gray-600 text-sm mb-3 line-clamp-2">
                        {description}
                    </p>
                }
                
                <div class="flex justify-between items-center mb-3">
                    <div class="text-2xl font-bold text-purple-600">
                        {format!("{:.0} تومان", product.price)}
                    </div>
                    <div class="text-sm text-gray-500">
                        {"موجودی: "} {product.stock}
                    </div>
                </div>
                
                // Product Metadata
                <div class="text-xs text-gray-400 mb-3">
                    {"آخرین بروزرسانی: "} {product.updated_at.format("%Y/%m/%d").to_string()}
                </div>
                
                // Action Buttons
                if props.show_actions {
                    <div class="flex space-x-2 space-x-reverse">
                        <button 
                            onclick={handle_edit}
                            class="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-sm py-2 px-3 rounded-md transition-colors duration-200"
                        >
                            {"ویرایش"}
                        </button>
                        <button 
                            onclick={handle_delete}
                            class="flex-1 bg-red-600 hover:bg-red-700 text-white text-sm py-2 px-3 rounded-md transition-colors duration-200"
                        >
                            {"حذف"}
                        </button>
                    </div>
                } else {
                    <button class="w-full bg-purple-600 hover:bg-purple-700 text-white text-sm py-2 px-3 rounded-md transition-colors duration-200">
                        {"افزودن به سبد خرید"}
                    </button>
                }
            </div>
        </div>
    }
}
