use yew::prelude::*;
use web_sys::MouseEvent;
use crate::models::CartItem;
use crate::components::CartSummary;

#[function_component(Cart)]
pub fn cart() -> Html {
    let cart_items = use_state(Vec::<CartItem>::new);

    // Mock cart data (run once on mount)
    {
        let cart_items = cart_items.clone();
        use_effect_with_deps(move |_| {
            // Mock cart items
            let mock_items = vec![
                CartItem {
                    id: Some(1),
                    uniform_id: 1,
                    uniform: None, // In real app, this would be fetched
                    quantity: 2,
                    price_at_time: 25.99,
                },
                CartItem {
                    id: Some(2),
                    uniform_id: 2,
                    uniform: None,
                    quantity: 1,
                    price_at_time: 35.99,
                },
            ];
            cart_items.set(mock_items);

            || ()
        }, ());
    }

    let on_checkout = Callback::from(move |_: MouseEvent| {
        // TODO: Implement checkout logic
        web_sys::console::log_1(&"Proceeding to checkout".into());
    });

    let on_remove_item = {
        let cart_items = cart_items.clone();
        Callback::from(move |uniform_id: i32| {
            let mut items = (*cart_items).clone();
            items.retain(|item| item.uniform_id != uniform_id);
            cart_items.set(items);
        })
    };

    let on_update_quantity = {
        let cart_items = cart_items.clone();
        Callback::from(move |(uniform_id, new_quantity): (i32, i32)| {
            let mut items = (*cart_items).clone();
            if let Some(item) = items.iter_mut().find(|item| item.uniform_id == uniform_id) {
                item.quantity = new_quantity;
            }
            cart_items.set(items);
        })
    };

    html! {
        <div class="cart-page">
            <div class="cart-container">
                <CartSummary
                    items={(*cart_items).clone()}
                    on_checkout={on_checkout}
                    on_remove_item={on_remove_item}
                    on_update_quantity={on_update_quantity}
                />
            </div>
        </div>
    }
}