use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::schools)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct School {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub contact_number: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::schools)]
pub struct NewSchool {
    pub name: String,
    pub address: Option<String>,
    pub contact_number: Option<String>,
}
