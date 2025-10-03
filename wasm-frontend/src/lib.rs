use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-purple-50 via-blue-50 to-indigo-100" dir="rtl">
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
                    </div>
                </div>
            </header>

            <main class="py-20 px-4">
                <div class="container mx-auto text-center">
                    <h2 class="text-5xl md:text-6xl font-bold text-gray-900 mb-6 leading-tight">
                        {"پلتفرم تجارت الکترونیک"}
                        <span class="block bg-gradient-to-r from-purple-600 to-blue-600 bg-clip-text text-transparent">
                            {"نسل جدید"}
                        </span>
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto leading-relaxed">
                        {"سیستم جامع مدیریت فروشگاه آنلاین با قابلیت‌های پیشرفته برای فروشندگان و مشتریان."}
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center items-center">
                        <button class="bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-700 hover:to-blue-700 text-white text-lg px-8 py-3 rounded-lg">
                            {"شروع کنید"}
                        </button>
                        <button class="border border-purple-200 hover:bg-purple-50 text-lg px-8 py-3 rounded-lg">
                            {"مشاهده دمو"}
                        </button>
                    </div>
                </div>
            </main>

            <section class="py-16 px-4 bg-white/50">
                <div class="container mx-auto">
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                        <div class="text-center">
                            <div class="text-3xl font-bold text-gray-900 mb-2">{"500+"}</div>
                            <div class="text-gray-600">{"فروشندگان فعال"}</div>
                        </div>
                        <div class="text-center">
                            <div class="text-3xl font-bold text-gray-900 mb-2">{"10K+"}</div>
                            <div class="text-gray-600">{"محصولات"}</div>
                        </div>
                        <div class="text-center">
                            <div class="text-3xl font-bold text-gray-900 mb-2">{"1K+"}</div>
                            <div class="text-gray-600">{"سفارشات روزانه"}</div>
                        </div>
                        <div class="text-center">
                            <div class="text-3xl font-bold text-gray-900 mb-2">{"98%"}</div>
                            <div class="text-gray-600">{"رضایت مشتریان"}</div>
                        </div>
                    </div>
                </div>
            </section>

            <footer class="bg-gray-900 text-white py-12 px-4">
                <div class="container mx-auto text-center">
                    <p>{"© 2024 پلتفرم پما. تمامی حقوق محفوظ است."}</p>
                </div>
            </footer>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
