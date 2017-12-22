extern crate barkbin_backend as barkbin;
extern crate diesel;

use self::barkbin::db::{establish_connection, create_bark};

fn main() {
    let connection = establish_connection().expect("Failed to open connection.");

    let bark_id = create_bark(&connection, "test.txt", "hello world");
    println!("Saved bark with id {:?}", bark_id);
}
