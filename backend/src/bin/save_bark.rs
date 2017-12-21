extern crate barkbin_backend as barkbin;
extern crate diesel;

use self::barkbin::db::models::*;
use self::barkbin::db::{establish_connection, create_bark};
use self::diesel::prelude::*;

fn main() {
    use barkbin::db::schema::barks::dsl::*;

    let connection = establish_connection().expect("Failed to open connection.");

    let bark_id = create_bark(&connection, "test.txt", "hello world");
    println!("Saved bark with id {}", bark_id);
}
