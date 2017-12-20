use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

// TODO: Use error-chain or failure here.
pub fn establish_connection() -> Result<SqliteConnection, String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set.")?;

    SqliteConnection::establish(&database_url)
        .map_err(|e| format!("Error connecting to {}: {}", database_url, e))
}