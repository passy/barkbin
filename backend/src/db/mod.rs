extern crate token_phrase;

use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::Error::DatabaseError;
use dotenv::dotenv;
use std::env;
use failure::{Error, err_msg};
use chrono::Utc;

pub mod schema;
pub mod models;
pub mod errors;

pub fn establish_connection() -> Result<SqliteConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| err_msg("DATABASE_URL must be set."))?;

    SqliteConnection::establish(&database_url)
        .map_err(|e| format_err!("Failed to open database {}: {}", database_url, e))
}

pub fn create_bark<'a>(conn: &SqliteConnection, filename: &'a str, body: &'a str) -> Result<usize, errors::DBError> {
    use db::schema::barks;
    use diesel::result::DatabaseErrorKind::*;

    // TODO: Configure the number of attempts somehow?
    for _ in 0 .. 4 {
        let new_bark = models::NewBark {
            filename: filename,
            body: body,
            slug: &token_phrase::generate_slug(2, (1 .. 1000))?,
            datetime: &Utc::now().naive_utc(),
        };
        let res = diesel::insert_into(barks::table)
            .values(&new_bark)
            .execute(conn);

        match res {
            Err(err) => match err {
                DatabaseError(UniqueViolation, _) => continue,
                err => return Err(err.into()),
            }
            Ok(o) => return Ok(o),
        };
    }

    Err(errors::DBError::UniquenessSaveError)
}
