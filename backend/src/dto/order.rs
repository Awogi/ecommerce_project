use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderRequest {
    pub user_id: i32,
    pub items: Vec<CreateOrderItemRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderItemRequest {
    pub uniform_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub status: String,
    pub created_at: String,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItemResponse {
    pub id: i32,
    pub uniform_id: i32,
    pub quantity: i32,
    pub price: f64,
}
