use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use failure::{Error, err_msg};

// TODO: Use error-chain or failure here.
pub fn establish_connection() -> Result<SqliteConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| err_msg("DATABASE_URL must be set."))?;

    SqliteConnection::establish(&database_url)
        .map_err(|e| format_err!("Failed to open database {}: {}", database_url, e))
}