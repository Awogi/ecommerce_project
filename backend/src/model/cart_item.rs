use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::cart_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CartItem {
    pub id: i32,
    pub user_id: i32,
    pub uniform_id: i32,
    pub quantity: i32,
    pub added_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::cart_items)]
pub struct NewCartItem {
    pub user_id: i32,
    pub uniform_id: i32,
    pub quantity: Option<i32>,
}
