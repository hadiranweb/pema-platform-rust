use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-container">
                <div class="footer-section">
                    <h3>{"پلتفرم پما"}</h3>
                    <p>{"سرمایه‌گذاری هوشمند در نقره‌های ارزشمند"}</p>
                </div>
                
                <div class="footer-section">
                    <h4>{"خدمات"}</h4>
                    <ul>
                        <li><a href="#products">{"محصولات نقره"}</a></li>
                        <li><a href="#investment">{"سرمایه‌گذاری"}</a></li>
                        <li><a href="#analytics">{"تحلیل بازار"}</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>{"پشتیبانی"}</h4>
                    <ul>
                        <li><a href="#help">{"راهنما"}</a></li>
                        <li><a href="#contact">{"تماس با ما"}</a></li>
                        <li><a href="#faq">{"سوالات متداول"}</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>{"فناوری"}</h4>
                    <p>{"ساخته شده با Rust و WebAssembly"}</p>
                    <p>{"قدرت گرفته از Manus AI"}</p>
                </div>
            </div>
            
            <div class="footer-bottom">
                <p>{"© 2024 پلتفرم پما. تمامی حقوق محفوظ است."}</p>
            </div>
        </footer>
    }
}
