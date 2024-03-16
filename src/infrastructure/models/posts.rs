use diesel::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::infrastructure::models::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
}