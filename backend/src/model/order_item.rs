use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::order_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub uniform_id: i32,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::order_items)]
pub struct NewOrderItem {
    pub order_id: i32,
    pub uniform_id: i32,
    pub quantity: i32,
    pub price: f64,
}
