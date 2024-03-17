use crate::database::db::PgPool;
use diesel::r2d2::R2D2Connection;
use rocket::get;
use rocket::serde::json::Json;
use std::time::SystemTime;

use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    api_name: String,
    current_time: SystemTime,
    response: String,
}

#[get("/healthcheck")]
pub fn healthcheck() -> &'static str {
    "All good!"
}

#[get("/liveliness")]
pub fn liveliness(pool: &rocket::State<PgPool>) -> Json<MessageResponse> {
    let mut con = pool.get().expect("Failed to get DB connection from pool");

    con.ping().expect("Database Ping failed");

    Json::from(MessageResponse {
        response: "Service live and well".to_string(),
        api_name: "rust-test-api".to_string(),
        current_time: SystemTime::now(),
    })
}
