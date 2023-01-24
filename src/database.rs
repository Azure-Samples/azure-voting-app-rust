use diesel::{pg::PgConnection, prelude::*, r2d2::ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use log::{error, info};
use r2d2::Pool;
use std::env;
use std::time::Duration;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn setup() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Establishing database connection");
    let mut connection: PgConnection;
    loop {
        match PgConnection::establish(&database_url) {
            Ok(conn) => {
                connection = conn;
                break;
            }
            Err(_) => {
                error!("Failed to establish a database connection, retrying in 5 seconds");
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }

    info!("Apply database migrations");
    let _ = connection.run_pending_migrations(MIGRATIONS);

    info!("Setting up database connection pool");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool")
}
