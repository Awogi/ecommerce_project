use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
// price represented as f64 (maps to DOUBLE PRECISION)

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::uniforms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::uniforms)]
pub struct NewUniform {
    pub name: String,
    pub school_id: i32,
    pub grade_id: i32,
    pub category_id: i32,
    pub size: Option<String>,
    pub price: f64,
    pub stock_quantity: Option<i32>,
    pub image_url: Option<String>,
}
