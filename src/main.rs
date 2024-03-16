use diesel::{RunQueryDsl};
use rocket::{Build, get, launch, Rocket, routes};
use rocket::serde::{json::Json};

use diesel::prelude::*;
use rust_api::database::db::{DbFaring, PgPool};
use rust_api::database::models::posts::Post;
use rust_api::database::models::schema::posts::dsl::posts;
use rust_api::controllers::k8s::k8s::{healthcheck, liveliness};


// Example route handler using the database connection
#[get("/example")]
async fn example(pool: &rocket::State<PgPool>) -> Json<Vec<Post>>{

    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    // Use `conn` here to interact with the database

    let result = posts
        .select(Post::as_select())
        .limit(5)
        .load(&mut conn)
        .expect("Error loading data from table");

    Json(result)
}


#[launch]
fn rocket() -> Rocket<Build>{
    rocket::build()
        .attach(DbFaring)
        .mount("/k8s", routes![healthcheck, liveliness])
        .mount("/", routes![example])

}


