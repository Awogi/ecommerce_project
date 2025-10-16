use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePaymentRequest {
    pub order_id: i32,
    pub payment_method: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentResponse {
    pub id: i32,
    pub order_id: i32,
    pub payment_method: String,
    pub amount: f64,
    pub status: Option<String>,
    pub paid_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentSummaryResponse {
    pub total_paid: f64,
    pub pending: i64,
    pub failed: i64,
}
