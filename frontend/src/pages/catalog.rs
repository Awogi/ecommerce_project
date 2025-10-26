use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::models::{Uniform, School, Grade, UniformCategory, FilterState};
use crate::components::{ProductCard, FilterSidebar, Loading};

#[function_component(Catalog)]
pub fn catalog() -> Html {
    let uniforms = use_state(Vec::<Uniform>::new);
    let schools = use_state(Vec::<School>::new);
    let grades = use_state(Vec::<Grade>::new);
    let categories = use_state(Vec::<UniformCategory>::new);
    let filters = use_state(FilterState::default);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    // Fetch initial data once on mount
    {
        let uniforms = uniforms.clone();
        let schools = schools.clone();
        let grades = grades.clone();
        let categories = categories.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    // Mock data for now - in real app, these would be API calls
                    let mock_schools = vec![
                        School { id: 1, name: "Greenwood Elementary".to_string(), address: None, contact_info: None },
                        School { id: 2, name: "Riverside Middle School".to_string(), address: None, contact_info: None },
                        School { id: 3, name: "Summit High School".to_string(), address: None, contact_info: None },
                    ];

                    let mock_grades = vec![
                        Grade { id: 1, name: "Kindergarten".to_string(), school_id: 1 },
                        Grade { id: 2, name: "1st Grade".to_string(), school_id: 1 },
                        Grade { id: 3, name: "6th Grade".to_string(), school_id: 2 },
                        Grade { id: 4, name: "9th Grade".to_string(), school_id: 3 },
                    ];

                    let mock_categories = vec![
                        UniformCategory { id: 1, name: "Shirts".to_string(), description: None },
                        UniformCategory { id: 2, name: "Pants".to_string(), description: None },
                        UniformCategory { id: 3, name: "Blazers".to_string(), description: None },
                        UniformCategory { id: 4, name: "Accessories".to_string(), description: None },
                    ];

                    // Use static assets from frontend/static as mock images. Make sure these files exist
                    // in your frontend/static folder (e.g. navy_polo_300.png, khaki_pants_300.png, etc.).
                    // Map mock items to the images you added in frontend/static.
                    // If you rename files, update these paths accordingly.
                    let mock_uniforms = vec![
                        Uniform {
                            id: 1,
                            name: "Navy Blue Polo Shirt".to_string(),
                            school_id: 1,
                            grade_id: 1,
                            category_id: 1,
                            size: Some("M".to_string()),
                            price: 25.99,
                            stock_quantity: Some(15),
                            // Using a blue school image that exists in static (sanitized filename)
                            image_url: Some("/static/blue-school-uniform.jpg".to_string()),
                        },
                        Uniform {
                            id: 2,
                            name: "Khaki Dress Pants".to_string(),
                            school_id: 1,
                            grade_id: 1,
                            category_id: 2,
                            size: Some("M".to_string()),
                            price: 35.99,
                            stock_quantity: Some(8),
                            // fallback to a pants image you added (sanitized filename)
                            image_url: Some("/static/black-pant.jpg".to_string()),
                        },
                        Uniform {
                            id: 3,
                            name: "School Blazer".to_string(),
                            school_id: 2,
                            grade_id: 3,
                            category_id: 3,
                            size: Some("L".to_string()),
                            price: 89.99,
                            stock_quantity: Some(3),
                            image_url: Some("/static/blue-blezer.jpg".to_string()),
                        },
                        Uniform {
                            id: 4,
                            name: "White Button-Up Shirt".to_string(),
                            school_id: 3,
                            grade_id: 4,
                            category_id: 1,
                            size: Some("S".to_string()),
                            price: 28.99,
                            stock_quantity: Some(0),
                            image_url: Some("/static/white-shirt-male.jpg".to_string()),
                        },
                    ];

                    schools.set(mock_schools);
                    grades.set(mock_grades);
                    categories.set(mock_categories);
                    uniforms.set(mock_uniforms);
                    loading.set(false);
                });

                || ()
            },
            (),
        );
    }

    let on_filter_change = {
        let filters = filters.clone();
        Callback::from(move |new_filters: FilterState| {
            filters.set(new_filters);
        })
    };

    let on_add_to_cart = Callback::from(move |uniform: Uniform| {
        // TODO: Add to cart logic
        web_sys::console::log_1(&format!("Added to cart: {}", uniform.name).into());
    });

    // Filter uniforms based on current filters
    let filtered_uniforms = {
        let uniforms = (*uniforms).clone();
        let filters = (*filters).clone();
        
        uniforms.into_iter()
            .filter(|uniform| {
                // Filter by school
                if let Some(school_id) = filters.school_id {
                    if uniform.school_id != school_id {
                        return false;
                    }
                }

                // Filter by grade
                if let Some(grade_id) = filters.grade_id {
                    if uniform.grade_id != grade_id {
                        return false;
                    }
                }

                // Filter by category
                if let Some(category_id) = filters.category_id {
                    if uniform.category_id != category_id {
                        return false;
                    }
                }

                // Filter by search term
                if let Some(search_term) = &filters.search_term {
                    if !uniform.name.to_lowercase().contains(&search_term.to_lowercase()) {
                        return false;
                    }
                }

                true
            })
            .collect::<Vec<_>>()
    };

    if *loading {
        return html! { <Loading /> };
    }

    if let Some(error_message) = (*error).as_ref() {
        return html! {
            <div class="error-container">
                <h2>{"Error loading catalog"}</h2>
                <p>{error_message}</p>
            </div>
        };
    }

    html! {
        <div class="catalog-page">
            <div class="catalog-container">
                <aside class="catalog-sidebar">
                    <FilterSidebar
                        schools={(*schools).clone()}
                        grades={(*grades).clone()}
                        categories={(*categories).clone()}
                        filters={(*filters).clone()}
                        on_filter_change={on_filter_change}
                    />
                </aside>

                <main class="catalog-main">
                    <div class="catalog-header">
                        <h1>{"School Uniforms"}</h1>
                        <div class="catalog-meta">
                            <span class="results-count">
                                {format!("{} items found", filtered_uniforms.len())}
                            </span>
                            <div class="sort-controls">
                                <label>{"Sort by:"}</label>
                                <select class="sort-select">
                                    <option value="name">{"Name"}</option>
                                    <option value="price-low">{"Price: Low to High"}</option>
                                    <option value="price-high">{"Price: High to Low"}</option>
                                    <option value="newest">{"Newest"}</option>
                                </select>
                            </div>
                        </div>
                    </div>

                    {if filtered_uniforms.is_empty() {
                        html! {
                            <div class="no-results">
                                <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                                    <circle cx="11" cy="11" r="8"></circle>
                                    <path d="m21 21-4.35-4.35"></path>
                                </svg>
                                <h3>{"No uniforms found"}</h3>
                                <p>{"Try adjusting your filters or search terms to find what you're looking for."}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="products-grid">
                                {for filtered_uniforms.iter().map(|uniform| {
                                    html! {
                                        <ProductCard
                                            key={uniform.id}
                                            uniform={uniform.clone()}
                                            on_add_to_cart={on_add_to_cart.clone()}
                                        />
                                    }
                                })}
                            </div>
                        }
                    }}
                </main>
            </div>
        </div>
    }
}