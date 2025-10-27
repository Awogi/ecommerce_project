use yew::prelude::*;
use web_sys::MouseEvent;
use crate::models::{CartItem, AppState};
use crate::components::CartSummary;
use yew::prelude::UseStateHandle;

#[function_component(Cart)]
pub fn cart() -> Html {
    // Prefer reading cart items from global AppState context. Fall back to local mock if context absent.
    let app_ctx = use_context::<UseStateHandle<AppState>>();
    let cart_items_vec: Vec<CartItem> = if let Some(ref app_state) = app_ctx {
        (*app_state).cart_items.clone()
    } else {
        // Mock cart data (when no context provided)
        vec![
            CartItem { id: Some(1), uniform_id: 1, uniform: None, quantity: 2, price_at_time: 25.99 },
            CartItem { id: Some(2), uniform_id: 2, uniform: None, quantity: 1, price_at_time: 35.99 },
        ]
    };

    let on_checkout = Callback::from(move |_: MouseEvent| {
        // TODO: Implement checkout logic
        web_sys::console::log_1(&"Proceeding to checkout".into());
    });

    let on_remove_item = {
        let app_ctx = app_ctx.clone();
        Callback::from(move |uniform_id: i32| {
            if let Some(app_state) = &app_ctx {
                let mut new_state = <AppState as Clone>::clone(&*app_state);
                new_state.cart_items.retain(|item| item.uniform_id != uniform_id);
                app_state.set(new_state);
            }
        })
    };

    let on_update_quantity = {
        let app_ctx = app_ctx.clone();
        Callback::from(move |(uniform_id, new_quantity): (i32, i32)| {
            if let Some(app_state) = &app_ctx {
                let mut new_state = <AppState as Clone>::clone(&*app_state);
                if let Some(item) = new_state.cart_items.iter_mut().find(|item| item.uniform_id == uniform_id) {
                    item.quantity = new_quantity;
                }
                app_state.set(new_state);
            }
        })
    };

    html! {
        <div class="cart-page">
            <div class="cart-container">
                <CartSummary
                    items={cart_items_vec}
                    on_checkout={on_checkout}
                    on_remove_item={on_remove_item}
                    on_update_quantity={on_update_quantity}
                />
            </div>
        </div>
    }
}