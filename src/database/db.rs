use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::{
    fairing::{Fairing, Info, Kind},
    Build, Rocket,
};
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbFaring;

#[rocket::async_trait]
impl Fairing for DbFaring {
    fn info(&self) -> Info {
        Info {
            name: "Database Connection Fairing",
            kind: Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let db_pool = get_connection_pool()
            .await
            .expect("Failed to create database connection pool");
        Ok(rocket.manage(db_pool))
    }
}

pub async fn get_connection_pool() -> Result<PgPool, Box<dyn std::error::Error>> {
    let url = database_url_for_env();
    let manager = ConnectionManager::<PgConnection>::new(url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(std::time::Duration::from_secs(3))
        .build(manager)?;

    Ok(pool)
}

fn database_url_for_env() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL not set in environment")
}
