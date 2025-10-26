use yew::prelude::*;
use web_sys::MouseEvent;
use crate::models::CartItem;

#[derive(Properties, PartialEq)]
pub struct CartSummaryProps {
    pub items: Vec<CartItem>,
    pub on_checkout: Callback<MouseEvent>,
    pub on_remove_item: Callback<i32>,
    pub on_update_quantity: Callback<(i32, i32)>,
}

#[function_component(CartSummary)]
pub fn cart_summary(props: &CartSummaryProps) -> Html {
    let CartSummaryProps { items, on_checkout, on_remove_item, on_update_quantity } = props;

    let total_price: f64 = items.iter()
        .map(|item| item.price_at_time * item.quantity as f64)
        .sum();

    let total_items: i32 = items.iter().map(|item| item.quantity).sum();

    html! {
        <div class="cart-summary">
            <h2 class="cart-title">{"Shopping Cart"}</h2>
            
            {if items.is_empty() {
                html! {
                    <div class="empty-cart">
                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                            <path d="M3 3h2l.4 2M7 13h10l4-8H5.4m2.6 8L6 6H2m5 7v6a1 1 0 001 1h8a1 1 0 001-1v-6m-9 0h8"></path>
                        </svg>
                        <h3>{"Your cart is empty"}</h3>
                        <p>{"Add some uniforms to get started!"}</p>
                    </div>
                }
            } else {
                html! {
                    <div class="cart-content">
                        <div class="cart-items">
                            {for items.iter().map(|item| {
                                let uniform = item.uniform.as_ref();
                                let remove_onclick = {
                                    let on_remove = on_remove_item.clone();
                                    let uniform_id = item.uniform_id;
                                    Callback::from(move |_| on_remove.emit(uniform_id))
                                };

                                html! {
                                    <div key={item.uniform_id} class="cart-item">
                                        <div class="item-image">
                                            {if let Some(uniform) = uniform {
                                                html! {
                                                    <img 
                                                        src={uniform.image_url.clone().unwrap_or_else(|| "/api/placeholder/80/80".to_string())}
                                                        alt={uniform.name.clone()}
                                                    />
                                                }
                                            } else {
                                                html! { <div class="placeholder-image"></div> }
                                            }}
                                        </div>
                                        
                                        <div class="item-details">
                                            {if let Some(uniform) = uniform {
                                                html! {
                                                    <>
                                                        <h4 class="item-name">{&uniform.name}</h4>
                                                        {if let Some(size) = &uniform.size {
                                                            html! { <p class="item-size">{format!("Size: {}", size)}</p> }
                                                        } else {
                                                            html! {}
                                                        }}
                                                    </>
                                                }
                                            } else {
                                                html! { <h4 class="item-name">{"Unknown Item"}</h4> }
                                            }}
                                        </div>
                                        
                                        <div class="item-quantity">
                                            <label>{"Qty:"}</label>
                                            <input 
                                                type="number" 
                                                min="1" 
                                                value={item.quantity.to_string()}
                                                class="quantity-input"
                                            />
                                        </div>
                                        
                                        <div class="item-price">
                                            <span class="price">{format!("${:.2}", item.price_at_time)}</span>
                                            <span class="subtotal">{format!("${:.2}", item.price_at_time * item.quantity as f64)}</span>
                                        </div>
                                        
                                        <button class="remove-btn" onclick={remove_onclick}>
                                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <path d="M3 6h18M8 6V4a2 2 0 012-2h4a2 2 0 012 2v2m3 0v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6h14zM10 11v6M14 11v6"></path>
                                            </svg>
                                        </button>
                                    </div>
                                }
                            })}
                        </div>
                        
                        <div class="cart-footer">
                            <div class="cart-totals">
                                <div class="total-row">
                                    <span>{"Items:"}</span>
                                    <span>{total_items}</span>
                                </div>
                                <div class="total-row subtotal">
                                    <span>{"Subtotal:"}</span>
                                    <span>{format!("${:.2}", total_price)}</span>
                                </div>
                                <div class="total-row shipping">
                                    <span>{"Shipping:"}</span>
                                    <span>{"Free"}</span>
                                </div>
                                <div class="total-row total">
                                    <span>{"Total:"}</span>
                                    <span>{format!("${:.2}", total_price)}</span>
                                </div>
                            </div>
                            
                            <button 
                                class="checkout-btn" 
                                onclick={on_checkout.clone()}
                            >
                                {"Proceed to Checkout"}
                            </button>
                        </div>
                    </div>
                }
            }}
        </div>
    }
}