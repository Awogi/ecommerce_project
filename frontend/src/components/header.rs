use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <div class="header-container">
                <div class="logo-section">
                    <Link<Route> to={Route::Home} classes="logo-btn"> 
                        <div style="display:flex;align-items:center;gap:0.75rem">
                            <div class="logo-icon" aria-hidden="true">{
                                html!{
                                    <svg width="34" height="34" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                        <rect x="3" y="3" width="18" height="18" rx="4" fill="#0f766e" />
                                        <path d="M7 9h10M7 13h6" stroke="#fff" stroke-width="1.5"/>
                                    </svg>
                                }
                            }</div>
                            <h1 class="logo">{"UniForm Store"}</h1>
                        </div>
                    </Link<Route>>
                </div>

                <nav class="nav-menu">
                    <div class="dropdown">
                        <button class="nav-btn dropdown-toggle">{"Schools ▾"}</button>
                        <div class="dropdown-menu">
                            <button class="dropdown-item">{"St. Xavier's School"}</button>
                            <button class="dropdown-item">{"Budhanilkantha School"}</button>
                            <button class="dropdown-item">{"Rato Bangala School"}</button>
                            <button class="dropdown-item">{"Ullens School"}</button>
                            <button class="dropdown-item">{"Kathmandu University School"}</button>
                            <button class="dropdown-item">{"Shuvatara School"}</button>
                        </div>
                    </div>
                    <Link<Route> to={Route::Catalog} classes="nav-btn nav-link-no-underline">{"All Products"}</Link<Route>>
                </nav>

                <div class="search-center">
                    <div class="search-box center">
                        <input 
                            type="text" 
                            placeholder="Search uniforms..." 
                            class="search-input search-input-center"
                        />
                        <button class="search-btn">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="11" cy="11" r="7"></circle>
                                <path d="m21 21-4.35-4.35"></path>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="header-actions">
                    <Link<Route> to={Route::Login} classes="login-btn">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                    </Link<Route>>

                    <Link<Route> to={Route::Cart} classes="cart-btn">
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M3 3h2l.4 2M7 13h10l4-8H5.4m2.6 8L6 6H2m5 7v6a1 1 0 001 1h8a1 1 0 001-1v-6m-9 0h8"></path>
                        </svg>
                        <span class="cart-count">{"3"}</span>
                    </Link<Route>>
                </div>
            </div>
        </header>
    }
}