use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String, // "student", "parent", "admin"
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub role: Option<String>, // default "student"
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
}
