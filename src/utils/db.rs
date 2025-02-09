use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the connection manager for SQLite
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // Create the connection pool
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
