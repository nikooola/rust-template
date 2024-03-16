use crate::database::db::PgPool;
use diesel::r2d2::R2D2Connection;
use rocket::get;

#[get("/healthcheck")]
pub fn healthcheck() -> &'static str {
    "All good!"
}

#[get("/liveliness")]
pub fn liveliness(pool: &rocket::State<PgPool>) -> &'static str {
    let mut con = pool.get().expect("Failed to get DB connection from pool");

    con.ping().expect("Database Ping failed");

    "Service live and well"
}
