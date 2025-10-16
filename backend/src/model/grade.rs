use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::grades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Grade {
    pub id: i32,
    pub name: String,
    pub school_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::grades)]
pub struct NewGrade {
    pub name: String,
    pub school_id: i32,
}
