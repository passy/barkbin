extern crate token_phrase;

use token_phrase::generate_slug;

fn main() {
    let slug = generate_slug(2, (1..10000)).unwrap();
    println!("Random slug: {}", slug);
}
