use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::components::{button::Button, card::Card};

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let on_get_started = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    let on_view_products = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Products);
        })
    };

    html! {
        <div class="home-page">
            <section class="hero">
                <div class="hero-content">
                    <h1 class="hero-title">{"پلتفرم پما"}</h1>
                    <h2 class="hero-subtitle">{"سرمایه‌گذاری هوشمند در نقره‌های ارزشمند"}</h2>
                    <p class="hero-description">
                        {"با استفاده از فناوری Rust و هوش مصنوعی Manus AI، در دنیای نقره‌های گردآفرید سرمایه‌گذاری کنید"}
                    </p>
                    <div class="hero-actions">
                        <Button onclick={on_get_started} variant="primary" size="large">
                            {"شروع کنید"}
                        </Button>
                        <Button onclick={on_view_products} variant="secondary" size="large">
                            {"مشاهده محصولات"}
                        </Button>
                    </div>
                </div>
            </section>

            <section class="features">
                <div class="container">
                    <h2 class="section-title">{"ویژگی‌های پلتفرم"}</h2>
                    <div class="features-grid">
                        <Card title="نقره‌های گردآفرید" class="feature-card">
                            <p>{"محصولات نقره‌ای منحصر به فرد با طراحی دولایه و کیفیت بالا"}</p>
                        </Card>
                        
                        <Card title="سرمایه‌گذاری هوشمند" class="feature-card">
                            <p>{"مدل سهام مشتری‌محور با بازدهی جذاب و ریسک کنترل شده"}</p>
                        </Card>
                        
                        <Card title="فناوری پیشرفته" class="feature-card">
                            <p>{"ساخته شده با Rust و WebAssembly برای عملکرد بهینه"}</p>
                        </Card>
                        
                        <Card title="هوش مصنوعی Manus" class="feature-card">
                            <p>{"تحلیل بازار و پیش‌بینی قیمت با استفاده از AI پیشرفته"}</p>
                        </Card>
                        
                        <Card title="امنیت بالا" class="feature-card">
                            <p>{"حفاظت از سرمایه و اطلاعات شخصی با بالاترین استانداردها"}</p>
                        </Card>
                        
                        <Card title="شفافیت کامل" class="feature-card">
                            <p>{"رصد لحظه‌ای سرمایه‌گذاری و گزارش‌های دقیق"}</p>
                        </Card>
                    </div>
                </div>
            </section>

            <section class="stats">
                <div class="container">
                    <h2 class="section-title">{"آمار پلتفرم"}</h2>
                    <div class="stats-grid">
                        <div class="stat-item">
                            <h3 class="stat-number">{"۱۰۰+"}</h3>
                            <p class="stat-label">{"محصول نقره‌ای"}</p>
                        </div>
                        <div class="stat-item">
                            <h3 class="stat-number">{"۵۰۰+"}</h3>
                            <p class="stat-label">{"سرمایه‌گذار فعال"}</p>
                        </div>
                        <div class="stat-item">
                            <h3 class="stat-number">{"۹۵%"}</h3>
                            <p class="stat-label">{"رضایت مشتریان"}</p>
                        </div>
                        <div class="stat-item">
                            <h3 class="stat-number">{"۲۴/۷"}</h3>
                            <p class="stat-label">{"پشتیبانی آنلاین"}</p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="cta">
                <div class="container">
                    <h2>{"آماده شروع هستید؟"}</h2>
                    <p>{"همین امروز به جمع سرمایه‌گذاران پما بپیوندید"}</p>
                    <Button onclick={on_get_started} variant="primary" size="large">
                        {"ثبت‌نام رایگان"}
                    </Button>
                </div>
            </section>
        </div>
    }
}
