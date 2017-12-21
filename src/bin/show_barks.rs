extern crate barkbin;
extern crate diesel;

use self::barkbin::db::models::*;
use self::barkbin::db::establish_connection;
use self::diesel::prelude::*;

fn main() {
    use barkbin::db::schema::barks::dsl::*;

    let connection = establish_connection().expect("Failed to open connection.");
    let results = barks
        .limit(5)
        .load::<Bark>(&connection)
        .expect("Failed loading posts");

    println!("Displaying {} posts.", results.len());
    for post in results {
        println!("{}", post.filename);
        println!("----------\n");
        println!("{}", post.body);
    }
}
