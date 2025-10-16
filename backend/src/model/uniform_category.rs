use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::uniform_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UniformCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::uniform_categories)]
pub struct NewUniformCategory {
    pub name: String,
}
