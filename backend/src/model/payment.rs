use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::payments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Payment {
    pub id: i32,
    pub order_id: i32,
    pub payment_method: String,
    pub amount: f64,
    pub status: Option<String>,
    pub paid_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::payments)]
pub struct NewPayment {
    pub order_id: i32,
    pub payment_method: String,
    pub amount: f64,
    pub status: Option<String>,
}
