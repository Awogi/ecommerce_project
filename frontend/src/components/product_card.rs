use yew::prelude::*;
use web_sys::MouseEvent;
use crate::models::Uniform;

#[derive(Properties, PartialEq)]
pub struct ProductCardProps {
    pub uniform: Uniform,
    pub on_add_to_cart: Callback<Uniform>,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let ProductCardProps { uniform, on_add_to_cart } = props;
    
    let onclick = {
        let on_add_to_cart = on_add_to_cart.clone();
        let uniform = uniform.clone();
        Callback::from(move |_: MouseEvent| {
            on_add_to_cart.emit(uniform.clone());
        })
    };

    // Use an existing static asset as the fallback image. Adjust if you rename files.
    let default_image = "/static/white-shirt-male.jpg".to_string();
    let image_url = uniform.image_url.as_ref()
        .unwrap_or(&default_image);

    let stock_status = match uniform.stock_quantity {
        Some(qty) if qty > 0 => "In Stock",
        Some(_) => "Out of Stock",
        None => "Check Availability",
    };

    let is_out_of_stock = matches!(uniform.stock_quantity, Some(0));

    html! {
        <div class="product-card">
            <div class="product-image">
                <img src={image_url.clone()} alt={uniform.name.clone()} />
                {if is_out_of_stock {
                    html! { <div class="out-of-stock-badge">{"Out of Stock"}</div> }
                } else {
                    html! {}
                }}
            </div>
            
            <div class="product-info">
                <h3 class="product-name">{&uniform.name}</h3>
                
                <div class="product-details">
                    {if let Some(size) = &uniform.size {
                        html! { <span class="product-size">{format!("Size: {}", size)}</span> }
                    } else {
                        html! {}
                    }}
                </div>
                
                <div class="product-pricing">
                    <span class="product-price">{format!("${:.2}", uniform.price)}</span>
                    <span class={classes!("stock-status", if is_out_of_stock { "out-of-stock" } else { "in-stock" })}>
                        {stock_status}
                    </span>
                </div>
                
                <button 
                    class={classes!("add-to-cart-btn", if is_out_of_stock { "disabled" } else { "" })}
                    disabled={is_out_of_stock}
                    onclick={onclick}
                >
                    {if is_out_of_stock { "Out of Stock" } else { "Add to Cart" }}
                </button>
            </div>
        </div>
    }
}