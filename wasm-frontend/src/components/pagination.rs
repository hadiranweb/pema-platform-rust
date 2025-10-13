use yew::prelude::*;
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub current_page: u32,
    pub total_pages: u32,
    pub total_items: u32,
    pub limit: u32,
    pub on_page_change: Callback<u32>,
    #[prop_or_default]
    pub on_limit_change: Option<Callback<u32>>,
}

#[function_component(PaginationComponent)]
pub fn pagination_component(props: &PaginationProps) -> Html {
    let current_page = props.current_page;
    let total_pages = props.total_pages;
    let on_page_change = props.on_page_change.clone();
    let on_limit_change = props.on_limit_change.clone();

    let handle_previous = {
        let on_page_change = on_page_change.clone();
        Callback::from(move |_| {
            if current_page > 1 {
                on_page_change.emit(current_page - 1);
            }
        })
    };

    let handle_next = {
        let on_page_change = on_page_change.clone();
        Callback::from(move |_| {
            if current_page < total_pages {
                on_page_change.emit(current_page + 1);
            }
        })
    };

    let handle_first = {
        let on_page_change = on_page_change.clone();
        Callback::from(move |_| {
            on_page_change.emit(1);
        })
    };

    let handle_last = {
        let on_page_change = on_page_change.clone();
        Callback::from(move |_| {
            on_page_change.emit(total_pages);
        })
    };

    let handle_limit_change = {
        let on_limit_change = on_limit_change.clone();
        Callback::from(move |e: Event| {
            if let Some(callback) = &on_limit_change {
                if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                    if let Ok(new_limit) = target.value().parse::<u32>() {
                        callback.emit(new_limit);
                    }
                }
            }
        })
    };

    // Generate page numbers to display
    let page_numbers = {
        let mut pages = Vec::new();
        let start = if current_page <= 3 { 1 } else { current_page - 2 };
        let end = if current_page + 2 >= total_pages { total_pages } else { current_page + 2 };
        
        for i in start..=end {
            pages.push(i);
        }
        pages
    };

    let start_item = (current_page - 1) * props.limit + 1;
    let end_item = std::cmp::min(current_page * props.limit, props.total_items);

    html! {
        <div class="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200 sm:px-6">
            <div class="flex-1 flex justify-between sm:hidden">
                // Mobile pagination
                <button
                    onclick={handle_previous.clone()}
                    disabled={current_page <= 1}
                    class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"قبلی"}
                </button>
                <button
                    onclick={handle_next.clone()}
                    disabled={current_page >= total_pages}
                    class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"بعدی"}
                </button>
            </div>
            
            <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                <div class="flex items-center space-x-4 space-x-reverse">
                    <p class="text-sm text-gray-700">
                        {"نمایش "}
                        <span class="font-medium">{start_item}</span>
                        {" تا "}
                        <span class="font-medium">{end_item}</span>
                        {" از "}
                        <span class="font-medium">{props.total_items}</span>
                        {" نتیجه"}
                    </p>
                    
                    if let Some(_) = &on_limit_change {
                        <div class="flex items-center space-x-2 space-x-reverse">
                            <label class="text-sm text-gray-700">{"تعداد در صفحه:"}</label>
                            <select
                                onchange={handle_limit_change}
                                class="border border-gray-300 rounded-md text-sm py-1 px-2 focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                            >
                                <option value="10" selected={props.limit == 10}>{"10"}</option>
                                <option value="25" selected={props.limit == 25}>{"25"}</option>
                                <option value="50" selected={props.limit == 50}>{"50"}</option>
                                <option value="100" selected={props.limit == 100}>{"100"}</option>
                            </select>
                        </div>
                    }
                </div>
                
                <div>
                    <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
                        // First page button
                        if current_page > 3 {
                            <button
                                onclick={handle_first}
                                class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50"
                            >
                                {"1"}
                            </button>
                            if current_page > 4 {
                                <span class="relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700">
                                    {"..."}
                                </span>
                            }
                        }
                        
                        // Previous button
                        <button
                            onclick={handle_previous.clone()}
                            disabled={current_page <= 1}
                            class="relative inline-flex items-center px-2 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <span class="sr-only">{"قبلی"}</span>
                            {"‹"}
                        </button>
                        
                        // Page numbers
                        {for page_numbers.iter().map(|&page| {
                            let is_current = page == current_page;
                            let handle_page_click = {
                                let on_page_change = on_page_change.clone();
                                Callback::from(move |_| {
                                    on_page_change.emit(page);
                                })
                            };
                            
                            html! {
                                <button
                                    onclick={handle_page_click}
                                    class={if is_current {
                                        "z-10 bg-purple-50 border-purple-500 text-purple-600 relative inline-flex items-center px-4 py-2 border text-sm font-medium"
                                    } else {
                                        "bg-white border-gray-300 text-gray-500 hover:bg-gray-50 relative inline-flex items-center px-4 py-2 border text-sm font-medium"
                                    }}
                                >
                                    {page}
                                </button>
                            }
                        })}
                        
                        // Next button
                        <button
                            onclick={handle_next.clone()}
                            disabled={current_page >= total_pages}
                            class="relative inline-flex items-center px-2 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <span class="sr-only">{"بعدی"}</span>
                            {"›"}
                        </button>
                        
                        // Last page button
                        if current_page < total_pages - 2 {
                            if current_page < total_pages - 3 {
                                <span class="relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700">
                                    {"..."}
                                </span>
                            }
                            <button
                                onclick={handle_last}
                                class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50"
                            >
                                {total_pages}
                            </button>
                        }
                    </nav>
                </div>
            </div>
        </div>
    }
}
