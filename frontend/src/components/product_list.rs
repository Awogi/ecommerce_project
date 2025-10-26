// This file defines the ProductList component that displays a list of products.

use yew::prelude::*;
use crate::models::Product;

#[props]
pub struct ProductListProps {
    pub products: Vec<Product>,
}

#[function_component(ProductList)]
pub fn product_list(props: &ProductListProps) -> Html {
    let product_items = props.products.iter().map(|product| {
        html! {
            <div class="product-item">
                <h2>{ &product.name }</h2>
                <p>{ &product.description }</p>
                <p>{ format!("Price: ${:.2}", product.price) }</p>
                <button>{ "Add to Cart" }</button>
            </div>
        }
    });

    html! {
        <div class="product-list">
            { for product_items }
        </div>
    }
}