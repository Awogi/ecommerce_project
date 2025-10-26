use yew::prelude::*;
use crate::models::{User, Order};

#[function_component(Profile)]
pub fn profile() -> Html {
    let user = use_state(|| None::<User>);
    let orders = use_state(Vec::<Order>::new);
    let active_tab = use_state(|| "orders".to_string());

    // Mock user data (run once on mount)
    {
        let user = user.clone();
        let orders = orders.clone();
        use_effect_with_deps(move |_| {
            let mock_user = User {
                id: 1,
                full_name: "John Doe".to_string(),
                email: "john.doe@email.com".to_string(),
                role: "parent".to_string(),
            };
            user.set(Some(mock_user));

            let mock_orders = vec![
                Order {
                    id: 1,
                    user_id: 1,
                    total_amount: 87.97,
                    status: "Delivered".to_string(),
                    items: vec![], // Would be populated with actual items
                },
                Order {
                    id: 2,
                    user_id: 1,
                    total_amount: 45.99,
                    status: "Processing".to_string(),
                    items: vec![],
                },
            ];
            orders.set(mock_orders);

            || ()
        }, ());
    }

    let set_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="profile-page">
            <div class="profile-container">
                {if let Some(user_data) = (*user).as_ref() {
                    html! {
                        <>
                            <div class="profile-header">
                                <div class="profile-info">
                                    <div class="profile-avatar">
                                        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                            <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"></path>
                                            <circle cx="12" cy="7" r="4"></circle>
                                        </svg>
                                    </div>
                                    <div class="profile-details">
                                        <h1>{&user_data.full_name}</h1>
                                        <p class="profile-email">{&user_data.email}</p>
                                        <span class={classes!("profile-role", user_data.role.clone())}>
                                            {user_data.role.to_uppercase()}
                                        </span>
                                    </div>
                                </div>
                                <button class="edit-profile-btn">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"></path>
                                        <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                                    </svg>
                                    {"Edit Profile"}
                                </button>
                            </div>

                            <div class="profile-tabs">
                                <button 
                                    class={classes!("tab-btn", if *active_tab == "orders" { "active" } else { "" })}
                                    onclick={set_tab.reform(|_| "orders".to_string())}
                                >
                                    {"Order History"}
                                </button>
                                <button 
                                    class={classes!("tab-btn", if *active_tab == "account" { "active" } else { "" })}
                                    onclick={set_tab.reform(|_| "account".to_string())}
                                >
                                    {"Account Settings"}
                                </button>
                                <button 
                                    class={classes!("tab-btn", if *active_tab == "addresses" { "active" } else { "" })}
                                    onclick={set_tab.reform(|_| "addresses".to_string())}
                                >
                                    {"Addresses"}
                                </button>
                            </div>

                            <div class="profile-content">
                                {match (*active_tab).as_str() {
                                    "orders" => html! {
                                        <div class="orders-section">
                                            <h2>{"Order History"}</h2>
                                            {if (*orders).is_empty() {
                                                html! {
                                                    <div class="empty-state">
                                                        <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                                                            <path d="M16 4h2a2 2 0 012 2v14a2 2 0 01-2 2H6a2 2 0 01-2-2V6a2 2 0 012-2h2"></path>
                                                            <rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect>
                                                        </svg>
                                                        <h3>{"No orders yet"}</h3>
                                                        <p>{"When you make a purchase, your orders will appear here."}</p>
                                                    </div>
                                                }
                                            } else {
                                                html! {
                                                    <div class="orders-list">
                                                        {for (*orders).iter().map(|order| {
                                                            html! {
                                                                <div key={order.id} class="order-card">
                                                                    <div class="order-header">
                                                                        <div class="order-number">
                                                                            <span class="label">{"Order #"}</span>
                                                                            <span class="value">{order.id}</span>
                                                                        </div>
                                                                        <div class="order-status">
                                                                            <span class={classes!("status-badge", order.status.to_lowercase())}>
                                                                                {&order.status}
                                                                            </span>
                                                                        </div>
                                                                    </div>
                                                                    <div class="order-details">
                                                                        <div class="order-total">
                                                                            <span class="label">{"Total: "}</span>
                                                                            <span class="amount">{format!("${:.2}", order.total_amount)}</span>
                                                                        </div>
                                                                        <div class="order-actions">
                                                                            <button class="order-action-btn view">{"View Details"}</button>
                                                                            {if order.status == "Processing" {
                                                                                html! { <button class="order-action-btn cancel">{"Cancel Order"}</button> }
                                                                            } else {
                                                                                html! {}
                                                                            }}
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            }
                                                        })}
                                                    </div>
                                                }
                                            }}
                                        </div>
                                    },
                                    "account" => html! {
                                        <div class="account-section">
                                            <h2>{"Account Settings"}</h2>
                                            <div class="settings-form">
                                                <div class="form-group">
                                                    <label class="form-label">{"Full Name"}</label>
                                                    <input 
                                                        type="text" 
                                                        class="form-input" 
                                                        value={user_data.full_name.clone()}
                                                    />
                                                </div>
                                                <div class="form-group">
                                                    <label class="form-label">{"Email"}</label>
                                                    <input 
                                                        type="email" 
                                                        class="form-input" 
                                                        value={user_data.email.clone()}
                                                    />
                                                </div>
                                                <div class="form-group">
                                                    <label class="form-label">{"Role"}</label>
                                                    <select class="form-select" value={user_data.role.clone()}>
                                                        <option value="student">{"Student"}</option>
                                                        <option value="parent">{"Parent"}</option>
                                                    </select>
                                                </div>
                                                <div class="form-actions">
                                                    <button class="btn-primary">{"Save Changes"}</button>
                                                    <button class="btn-secondary">{"Change Password"}</button>
                                                </div>
                                            </div>
                                        </div>
                                    },
                                    "addresses" => html! {
                                        <div class="addresses-section">
                                            <h2>{"Saved Addresses"}</h2>
                                            <div class="empty-state">
                                                <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                                                    <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"></path>
                                                    <circle cx="12" cy="10" r="3"></circle>
                                                </svg>
                                                <h3>{"No addresses saved"}</h3>
                                                <p>{"Add a shipping address to speed up checkout."}</p>
                                                <button class="btn-primary">{"Add Address"}</button>
                                            </div>
                                        </div>
                                    },
                                    _ => html! {}
                                }}
                            </div>
                        </>
                    }
                } else {
                    html! {
                        <div class="loading-container">
                            <div class="loading-spinner"></div>
                            <p>{"Loading profile..."}</p>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}