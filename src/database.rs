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

    // first try to get database connection details from environment variables
    // if that fails, try to get them from a file in /mnt/secrets
    // if that fails, panic
    let database_user = match env::var("DATABASE_USER") {
        Ok(user) => user,
        Err(_) => {
            std::fs::read_to_string("/mnt/secrets-store/database-user")
                .unwrap_or_else(|_| "postgres".to_string())
        }
    };

    let database_password = match env::var("DATABASE_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            std::fs::read_to_string("/mnt/secrets-store/database-password")
                .expect("Failed to read database-password from /mnt/secrets-store")
        }
    };

    let database_server = match env::var("DATABASE_SERVER") {
        Ok(server) => server,
        Err(_) => {
            std::fs::read_to_string("/mnt/secrets-store/database-server")
                .expect("Failed to read database-server from /mnt/secrets-store")
        }
    };

    let database_url = format!(
        "postgres://{}:{}@{}",
        database_user, database_password, database_server
    );

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
