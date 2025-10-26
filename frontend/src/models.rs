use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct School {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub contact_info: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Grade {
    pub id: i32,
    pub name: String,
    pub school_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UniformCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Uniform {
    pub id: i32,
    pub name: String,
    pub school_id: i32,
    pub grade_id: i32,
    pub category_id: i32,
    pub size: Option<String>,
    pub price: f64,
    pub stock_quantity: Option<i32>,
    pub image_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CartItem {
    pub id: Option<i32>,
    pub uniform_id: i32,
    pub uniform: Option<Uniform>,
    pub quantity: i32,
    pub price_at_time: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_amount: f64,
    pub status: String,
    pub items: Vec<OrderItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub uniform_id: i32,
    pub uniform: Option<Uniform>,
    pub quantity: i32,
    pub price_at_time: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

// Application State
#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub current_user: Option<User>,
    pub selected_school: Option<School>,
    pub cart_items: Vec<CartItem>,
    pub schools: Vec<School>,
    pub uniforms: Vec<Uniform>,
    pub grades: Vec<Grade>,
    pub categories: Vec<UniformCategory>,
    pub filters: FilterState,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FilterState {
    pub school_id: Option<i32>,
    pub grade_id: Option<i32>,
    pub category_id: Option<i32>,
    pub price_range: Option<(f64, f64)>,
    pub size: Option<String>,
    pub search_term: Option<String>,
}