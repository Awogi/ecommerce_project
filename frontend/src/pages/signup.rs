use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::MouseEvent;
use crate::app::Route;

#[function_component(Signup)]
pub fn signup() -> Html {
    let navigator = use_navigator().unwrap();
    let go_to_login = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&Route::Login))
    };

    html! {
        <div class="auth-page" style="background: linear-gradient(180deg, #eef6f6 0%, #f6fbfb 100%); min-height:100vh; display:flex; align-items:flex-start; justify-content:center; padding-top:48px;">
            <div class="auth-container" style="max-width:720px; width:100%;">
                <div style="text-align:center; margin-bottom:12px; grid-column:1 / -1;">
                    <div style="display:inline-flex; align-items:center; gap:0.75rem;">
                        <div style="width:48px; height:48px; background:#0f776e; border-radius:10px; display:flex; align-items:center; justify-content:center;">
                            { html!(<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5"><path d="M12 2L3 7l9 5 9-5-9-5z"></path><path d="M3 17l9 5 9-5"/></svg>) }
                        </div>
                        <h1 class="logo" style="color:var(--text-primary);">{"UniForm Store"}</h1>
                    </div>
                </div>

                <div class="auth-card" style="grid-column:1 / -1;">
                    <div class="auth-header">
                        <h1>{"Create an account"}</h1>
                        <p style="color:var(--text-secondary);">{"Enter your information to get started"}</p>
                    </div>

                    <form class="auth-form">
                        <div style="display:grid; grid-template-columns:1fr 1fr; gap:1rem; margin-bottom:1rem;">
                            <div>
                                <label class="form-label">{"First Name"}</label>
                                <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 12a5 5 0 100-10 5 5 0 000 10z"></path><path d="M2 22a10 10 0 0120 0"></path></svg>
                                    <input class="form-input" type="text" placeholder="John" style="border:none; background:transparent; width:100%" />
                                </div>
                            </div>

                            <div>
                                <label class="form-label">{"Last Name"}</label>
                                <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                    <input class="form-input" type="text" placeholder="Doe" style="border:none; background:transparent; width:100%" />
                                </div>
                            </div>
                        </div>

                        <div style="margin-bottom:1rem;">
                            <label class="form-label">{"Email"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 8l9 6 9-6"></path><path d="M21 19H3v-8"></path></svg>
                                <input class="form-input" type="email" placeholder="john@example.com" style="border:none; background:transparent; width:100%" />
                            </div>
                        </div>

                        <div style="margin-bottom:1rem;">
                            <label class="form-label">{"Phone Number"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M22 16.92V21a1 1 0 01-1.11 1 19.86 19.86 0 01-8.63-3.16 19.5 19.5 0 01-6-6A19.86 19.86 0 012 3.11 1 1 0 013 2h4.09a1 1 0 01.97.757l.7 2.8a1 1 0 01-.24.95L7.6 9.67a13 13 0 006 6l2.36-1.85a1 1 0 01.95-.24l2.8.7A1 1 0 0122 16.92z"></path></svg>
                                <input class="form-input" type="tel" placeholder="+977 98XXXXXXXX" style="border:none; background:transparent; width:100%" />
                            </div>
                        </div>

                        <div style="margin-bottom:1rem;">
                            <label class="form-label">{"Password"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="11" width="18" height="11" rx="2"></rect><path d="M7 11V7a5 5 0 0110 0v4"></path></svg>
                                <input class="form-input" type="password" placeholder="Create a password" style="border:none; background:transparent; width:100%" />
                                <button type="button" style="background:none; border:none; color:var(--text-secondary);">{ html!(<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2.5 12s3-7 9.5-7 9.5 7 9.5 7-3 7-9.5 7S2.5 12 2.5 12z"></path><circle cx="12" cy="12" r="3"></circle></svg>) }</button>
                            </div>
                        </div>

                        <div style="margin-bottom:1rem;">
                            <label class="form-label">{"Confirm Password"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="11" width="18" height="11" rx="2"></rect><path d="M7 11V7a5 5 0 0110 0v4"></path></svg>
                                <input class="form-input" type="password" placeholder="Confirm your password" style="border:none; background:transparent; width:100%" />
                                <button type="button" style="background:none; border:none; color:var(--text-secondary);">{ html!(<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2.5 12s3-7 9.5-7 9.5 7 9.5 7-3 7-9.5 7S2.5 12 2.5 12z"></path><circle cx="12" cy="12" r="3"></circle></svg>) }</button>
                            </div>
                        </div>

                        <label style="display:flex; align-items:center; gap:0.5rem; margin-bottom:1rem;">
                            <input type="checkbox" />
                            <span style="color:var(--text-secondary);">{"I agree to the "}<a href="#" style="color:var(--primary-color); text-decoration:underline;">{"Terms & Conditions"}</a>{" and "}<a href="#" style="color:var(--primary-color); text-decoration:underline;">{"Privacy Policy"}</a></span>
                        </label>

                        <div style="margin-bottom:1rem;">
                            <button class="auth-submit-btn" type="button" style="width:100%; background:#15786f; border-radius:10px; padding:0.95rem 1rem;">{"Create Account"}</button>
                        </div>

                        <div style="display:flex; align-items:center; gap:1rem; margin:1rem 0; align-self:stretch;">
                            <div style="flex:1; height:1px; background:var(--border-color);"></div>
                            <div style="color:var(--text-secondary); font-size:0.85rem; white-space:nowrap;">{"OR CONTINUE WITH"}</div>
                            <div style="flex:1; height:1px; background:var(--border-color);"></div>
                        </div>

                        <div style="display:flex; gap:1rem;">
                            <button type="button" class="btn-primary" style="flex:1; background:white; border:1px solid var(--border-color); color:var(--text-primary);">{ html!(<span style="display:inline-flex;align-items:center;gap:0.5rem;"><svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 12.3c0-5-4-9.1-9-9.1S3 7.3 3 12.3c0 4.6 3.4 8.4 7.8 9.1v-6.4H8.9v-2.7h1.9V9.3c0-1.9 1.1-3 2.8-3 .8 0 1.6.1 1.6.1v1.8h-1c-1 0-1.3.6-1.3 1.2v1.4h2.3l-.4 2.7h-1.9V21c4.4-.7 7.8-4.5 7.8-9.1z"></path></svg><span>{" Google"}</span></span>) }</button>
                            <button type="button" class="btn-primary" style="flex:1; background:white; border:1px solid var(--border-color); color:var(--text-primary);">{ html!(<span style="display:inline-flex;align-items:center;gap:0.5rem;"><svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z"></path></svg><span>{" Facebook"}</span></span>) }</button>
                        </div>

                        <div style="text-align:center; margin-top:1rem;">
                            <span style="color:var(--text-secondary);">{"Already have an account? "}</span>
                            <button type="button" style="background:none; border:none; color:var(--primary-color); font-weight:700;" onclick={go_to_login}>{"Sign in"}</button>
                        </div>

                    </form>
                </div>
            </div>
        </div>
    }
}
