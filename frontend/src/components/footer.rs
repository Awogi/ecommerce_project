use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-container">
                <div class="footer-section">
                    <h3>{"SchoolWear"}</h3>
                    <p>{"Your trusted source for quality school uniforms. We provide comfortable, durable, and stylish uniforms for students of all ages."}</p>
                    <div class="social-links">
                        <a href="#" class="social-link">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"/>
                            </svg>
                        </a>
                        <a href="#" class="social-link">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M22.46 6c-.77.35-1.6.58-2.46.69.88-.53 1.56-1.37 1.88-2.38-.83.5-1.75.85-2.72 1.05C18.37 4.5 17.26 4 16 4c-2.35 0-4.27 1.92-4.27 4.29 0 .34.04.67.11.98C8.28 9.09 5.11 7.38 3 4.79c-.37.63-.58 1.37-.58 2.15 0 1.49.75 2.81 1.91 3.56-.71 0-1.37-.2-1.95-.5v.03c0 2.08 1.48 3.82 3.44 4.21a4.22 4.22 0 0 1-1.93.07 4.28 4.28 0 0 0 4 2.98 8.521 8.521 0 0 1-5.33 1.84c-.34 0-.68-.02-1.02-.06C3.44 20.29 5.7 21 8.12 21 16 21 20.33 14.46 20.33 8.79c0-.19 0-.37-.01-.56.84-.6 1.56-1.36 2.14-2.23z"/>
                            </svg>
                        </a>
                        <a href="#" class="social-link">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12.017 0C5.396 0 .029 5.367.029 11.987c0 5.079 3.158 9.417 7.618 11.174-.105-.949-.199-2.403.041-3.439.219-.937 1.406-5.957 1.406-5.957s-.359-.72-.359-1.781c0-1.663.967-2.911 2.168-2.911 1.024 0 1.518.769 1.518 1.688 0 1.029-.653 2.567-.992 3.992-.285 1.193.6 2.165 1.775 2.165 2.128 0 3.768-2.245 3.768-5.487 0-2.861-2.063-4.869-5.008-4.869-3.41 0-5.409 2.562-5.409 5.199 0 1.033.394 2.143.889 2.747.099.12.112.225.085.347-.09.375-.293 1.199-.334 1.363-.053.225-.402.21-.402.21-.148-.064-2.423-1.098-2.423-4.41 0-3.766 2.735-7.229 7.881-7.229 4.135 0 7.35 2.947 7.35 6.875 0 4.104-2.588 7.413-6.188 7.413-1.21 0-2.35-.63-2.741-1.378l-.748 2.853c-.271 1.043-1.002 2.35-1.492 3.146C9.57 23.812 10.763 24.009 12.017 24.009c6.624-.001 11.99-5.367 11.99-11.988C24.007 5.367 18.641.001.017 0z.017 0"/>
                            </svg>
                        </a>
                    </div>
                </div>

                <div class="footer-section">
                    <h4>{"Quick Links"}</h4>
                    <ul class="footer-links">
                        <li><a href="#">{"Home"}</a></li>
                        <li><a href="#">{"Shop"}</a></li>
                        <li><a href="#">{"Size Guide"}</a></li>
                        <li><a href="#">{"Schools"}</a></li>
                        <li><a href="#">{"Contact Us"}</a></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h4>{"Customer Service"}</h4>
                    <ul class="footer-links">
                        <li><a href="#">{"FAQ"}</a></li>
                        <li><a href="#">{"Shipping Info"}</a></li>
                        <li><a href="#">{"Returns & Exchanges"}</a></li>
                        <li><a href="#">{"Track Your Order"}</a></li>
                        <li><a href="#">{"Help Center"}</a></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h4>{"Contact Info"}</h4>
                    <div class="contact-info">
                        <p>
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"></path>
                                <circle cx="12" cy="10" r="3"></circle>
                            </svg>
                            {"123 School Street, Education City"}
                        </p>
                        <p>
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6 19.79 19.79 0 01-3.07-8.67A2 2 0 014.11 2h3a2 2 0 012 1.72 12.84 12.84 0 00.7 2.81 2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45 12.84 12.84 0 002.81.7A2 2 0 0122 16.92z"></path>
                            </svg>
                            {"(555) 123-4567"}
                        </p>
                        <p>
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                <polyline points="22,6 12,13 2,6"></polyline>
                            </svg>
                            {"support@schoolwear.com"}
                        </p>
                    </div>
                </div>
            </div>

            <div class="footer-bottom">
                <div class="footer-container">
                    <p>{"© 2024 SchoolWear. All rights reserved."}</p>
                    <div class="footer-legal">
                        <a href="#">{"Privacy Policy"}</a>
                        <a href="#">{"Terms of Service"}</a>
                        <a href="#">{"Cookie Policy"}</a>
                    </div>
                </div>
            </div>
        </footer>
    }
}