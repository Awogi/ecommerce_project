use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UniformResponse {
    pub id: i32,
    pub name: String,
    pub school_id: i32,
    pub grade_id: i32,
    pub category_id: i32,
    pub size: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
    pub image_url: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUniformRequest {
    pub name: String,
    pub school_id: i32,
    pub grade_id: i32,
    pub category_id: i32,
    pub size: Option<String>,
    pub price: f64,
    pub stock_quantity: Option<i32>,
    pub image_url: Option<String>,
}
