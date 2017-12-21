use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use failure::{Error, err_msg};

pub mod schema;
pub mod models;

pub fn establish_connection() -> Result<SqliteConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| err_msg("DATABASE_URL must be set."))?;

    SqliteConnection::establish(&database_url)
        .map_err(|e| format_err!("Failed to open database {}: {}", database_url, e))
}

pub fn create_bark<'a>(conn: &SqliteConnection, filename: &'a str, body: &'a str) -> usize {
    use db::schema::barks;

    let new_bark = models::NewBark {
        filename: filename,
        body: body,
    };

    diesel::insert_into(barks::table)
        .values(&new_bark)
        .execute(conn)
        // TODO: failure.
        .expect("Error saving bark.")
}
