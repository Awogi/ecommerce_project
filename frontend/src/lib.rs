use yew::prelude::*;
use shared::Product;

#[function_component(App)]
fn app() -> Html {
    let products = vec![
        Product::new(1, "White Shirt", "Grade 9", "M", 1999),
        Product::new(2, "School Blazer", "Grade 12", "L", 4999),
    ];

    html! {
        <div>
            <h1>{"School Uniform Store"}</h1>
            <ul>
                { for products.iter().map(|p| html!{
                    <li>{ format!("{} ({} - {}) - ${:.2}", p.name, p.grade, p.size, p.price_cents as f64 / 100.0) }</li>
                }) }
            </ul>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}
