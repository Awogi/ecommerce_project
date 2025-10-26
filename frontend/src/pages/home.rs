use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::MouseEvent;
use crate::app::Route;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use js_sys;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();
    
    // Navigate to catalog normally, but for unauthenticated users open signup the first time
    // they click Shop Now. We use localStorage key `seen_shop_cta` to track the first click.
    let shop_navigate = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    // check if auth token exists
                    match storage.get_item("auth_token") {
                        Ok(Some(_token)) => {
                            // logged in -> catalog
                            navigator.push(&Route::Catalog);
                        }
                        _ => {
                            // not logged in -> check seen flag
                            match storage.get_item("seen_shop_cta") {
                                Ok(Some(_)) => {
                                    // already seen -> go to catalog
                                    navigator.push(&Route::Catalog);
                                }
                                _ => {
                                    // first time -> set flag and go to signup
                                    let _ = storage.set_item("seen_shop_cta", "1");
                                    navigator.push(&Route::Signup);
                                }
                            }
                        }
                    }
                    return;
                }
            }

            // fallback: go to catalog
            navigator.push(&Route::Catalog);
        })
    };

    // reuse shop_navigate for CTA
    let go_to_catalog = shop_navigate.clone();
    let go_to_catalog_cta = shop_navigate.clone();

    // Handlers for category buttons
    let shop_shirts = shop_navigate.clone();

    let shop_bottoms = shop_navigate.clone();

    let shop_outerwear = shop_navigate.clone();

    let shop_accessories = shop_navigate.clone();

    // Handler for size guide button (for now, just navigate to catalog)
    let show_size_guide = {
        let navigator = navigator.clone();
        Callback::from(move |_: MouseEvent| navigator.push(&Route::Catalog))
    };

    // Handler for school cards - could navigate to school-specific catalog later
    let view_school_uniforms = shop_navigate.clone();

    // Reveal-on-scroll: use a small JS snippet to create an IntersectionObserver that
    // adds the 'revealed' class to the #schools-section when it enters the viewport.
    {
        use_effect_with_deps(
            move |_| {
                                if let Some(_window) = web_sys::window() {
                                        let code = r#"(function(){
                        const el = document.getElementById('schools-section');
                        if(!el) return;
                        const obs = new IntersectionObserver((entries)=>{
                          entries.forEach(e=>{
                            if(e.isIntersecting){
                              e.target.classList.add('revealed');
                              obs.unobserve(e.target);
                            }
                          });
                        }, { threshold: 0.12 });
                        obs.observe(el);
                    })();"#;
                                        // use js_sys::eval to run the small JS snippet
                                        let _ = js_sys::eval(code);
                }

                || {}
            },
            (),
        );
    }

    html! {
        <div class="home-page">
            // Hero Section - updated copy and layout to match design
            <section class="hero-section">
                <div class="hero-content">
                    <div class="hero-text">
                        <span class="badge">{"New Collection 2025"}</span>
                        <h1 class="hero-title">{"Shop Your\nSchool\nUniforms Online"}</h1>
                        <p class="hero-description">{"Quality uniforms for every grade. Fast delivery, easy returns, and affordable prices for students across Nepal."}</p>
                        <div class="hero-actions">
                            <button class="cta-primary" onclick={go_to_catalog}>
                                {"Shop Now"}
                            </button>
                            <button class="cta-secondary" onclick={show_size_guide}>
                                {"Learn More"}
                            </button>
                        </div>
                    </div>
                    <div class="hero-image">
                        // Use the homepage image you added in frontend/static
                        <img src="/static/homepage-image.jpg" alt="Students in school uniforms" />
                    </div>
                </div>
            </section>

            // Shop by School - this section is visually below the fold so it appears after scrolling
            <section id="schools-section" class="schools-section reveal-on-scroll">
                <div class="container">
                    <h2 class="section-title">{"Shop by School"}</h2>
                    <p class="section-description">{"Find the perfect uniform for your school"}</p>

                    <div class="school-cards">
                        <div class="school-card large" onclick={view_school_uniforms.clone()}>
                            <div class="school-image">
                                <img src="/static/st-zavier-school.jpg" alt="St. Xavier's School" />
                            </div>
                            <div class="school-content">
                                <h3>{"St. Xavier's School"}</h3>
                                <p class="meta">{"Blue & White • All Grades"}</p>
                                <a class="view-link" onclick={view_school_uniforms.clone()}>{"View Collection →"}</a>
                            </div>
                        </div>

                        <div class="school-card large" onclick={view_school_uniforms.clone()}>
                            <div class="school-image">
                                <img src="/static/budanilakhantha-school.jpg" alt="Budhanilkantha School" />
                            </div>
                            <div class="school-content">
                                <h3>{"Budhanilkantha School"}</h3>
                                <p class="meta">{"Maroon & White • All Grades"}</p>
                                <a class="view-link" onclick={view_school_uniforms.clone()}>{"View Collection →"}</a>
                            </div>
                        </div>

                        <div class="school-card large" onclick={view_school_uniforms.clone()}>
                            <div class="school-image">
                                <img src="/static/rato-bangala-school.jpg" alt="Rato Bangala School" />
                            </div>
                            <div class="school-content">
                                <h3>{"Rato Bangala School"}</h3>
                                <p class="meta">{"Red & White • All Grades"}</p>
                                <a class="view-link" onclick={view_school_uniforms.clone()}>{"View Collection →"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // (observer mounted via use_effect_with_deps above)

            // Keep remaining sections (features/cta) as is but hide for brevity; they can remain below
        </div>
    }
}