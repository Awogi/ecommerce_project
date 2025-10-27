use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement, SubmitEvent, MouseEvent};
use crate::app::Route;
use crate::services::api::ApiService;
use crate::models::LoginRequest as LoginReq;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(String::new);
    let password = use_state(String::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let go_to_signup = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&Route::Signup))
    };

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    // avoid moving `loading` into the closure so the UI can still read it later
    let loading_handle = loading.clone();
    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let loading_handle = loading_handle.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if (*email).is_empty() || (*password).is_empty() {
                error.set(Some("Please provide email and password".into()));
                return;
            }
            loading_handle.set(true);
            error.set(None);

            // clone loading and state handles to use inside the async block
            let loading_inner = loading_handle.clone();
            let email_inner = email.clone();
            let password_inner = password.clone();
            let error_inner = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                    // call backend login
                    let login_req = LoginReq { email: (*email_inner).clone(), password: (*password_inner).clone() };
                    match ApiService::login(login_req).await {
                        Ok(user) => {
                            web_sys::console::log_1(&format!("Logged in user: {:?}", user).into());
                            // navigate to home or profile after login
                            // Note: navigation must happen on main thread, so use window.location for simplicity
                            // Here we just stop loading and optionally navigate
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Login error: {}", err).into());
                            error_inner.set(Some(err));
                        }
                    }
                    loading_inner.set(false);
            });
        })
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
                        <h1>{"Welcome back"}</h1>
                        <p style="color:var(--text-secondary);">{"Enter your credentials to access your account"}</p>
                    </div>

                    <form class="auth-form" onsubmit={on_submit}>
                        {if let Some(err) = &*error {
                            html!{ <div class="error-message">{ err.clone() }</div> }
                        } else { html!{} }}

                        <div style="margin-bottom:1rem;">
                            <label class="form-label">{"Email"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 8l9 6 9-6"></path><path d="M21 19H3v-8"></path></svg>
                                <input type="email" class="form-input" placeholder="john@example.com" value={(*email).clone()} onchange={on_email_change} style="border:none; background:transparent; width:100%" />
                            </div>
                        </div>

                        <div style="margin-bottom:0.75rem;">
                            <label class="form-label">{"Password"}</label>
                            <div class="form-input" style="display:flex; align-items:center; gap:0.5rem; padding:0.5rem 0.75rem; border-radius:8px; background:var(--bg-secondary);">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="11" width="18" height="11" rx="2"></rect><path d="M7 11V7a5 5 0 0110 0v4"></path></svg>
                                <input type="password" class="form-input" placeholder="Enter your password" value={(*password).clone()} onchange={on_password_change} style="border:none; background:transparent; width:100%" />
                                <button type="button" style="background:none; border:none; color:var(--text-secondary);">{ html!(<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M2.5 12s3-7 9.5-7 9.5 7 9.5 7-3 7-9.5 7S2.5 12 2.5 12z"></path><circle cx="12" cy="12" r="3"></circle></svg>) }</button>
                            </div>
                        </div>

                        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                            <label style="display:flex; align-items:center; gap:0.5rem;">
                                <input type="checkbox" />
                                <span style="color:var(--text-secondary);">{"Remember me"}</span>
                            </label>
                            <a href="#" class="forgot-password-link">{"Forgot password?"}</a>
                        </div>

                                                <button type="submit" class="auth-submit-btn" disabled={*loading}>
                                                        { if *loading {
                                                                html!{ <><div class="loading-spinner small"></div>{"Signing In..."}</> }
                                                            } else {
                                                                html!{"Sign In"}
                                                            } }
                                                </button>

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
                            <span style="color:var(--text-secondary);">{"Don't have an account? "}</span>
                            <button type="button" style="background:none; border:none; color:var(--primary-color); font-weight:700;" onclick={go_to_signup}>{"Sign up"}</button>
                        </div>
                    </form>
                </div>

                <div class="auth-features" style="grid-column:1 / -1;">
                    <h3>{"Why Shop With Us?"}</h3>
                    <div class="features-list">
                        <div class="feature-item">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"></path>
                            </svg>
                            <span>{"Premium quality uniforms"}</span>
                        </div>
                        <div class="feature-item">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z"></path>
                                <polyline points="9,22 9,12 15,12 15,22"></polyline>
                            </svg>
                            <span>{"Free shipping on orders $75+"}</span>
                        </div>
                        <div class="feature-item">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M22 11.08V12a10 10 0 11-5.93-9.14"></path>
                                <polyline points="22,4 12,14.01 9,11.01"></polyline>
                            </svg>
                            <span>{"30-day return policy"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}