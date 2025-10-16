use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub total_price: f64,
    pub status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub total_price: f64,
    pub status: Option<String>,
}
