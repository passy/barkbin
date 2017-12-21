extern crate rand;
#[macro_use]
extern crate lazy_static;

use std::io;
use rand::Rng;

static ADJECTIVES_STR: &'static str = include_str!("../data/adjectives.txt");
static ANIMALS_STR: &'static str = include_str!("../data/animals.txt");

lazy_static! {
    static ref ADJECTIVES: Vec<&'static str> = ADJECTIVES_STR.lines().collect();
    static ref ANIMALS: Vec<&'static str> = ANIMALS_STR.lines().collect();
}

pub fn generate_slug(num_adjectives: u8, id_range: std::ops::Range<u64>) -> io::Result<String> {
    let mut rng = rand::os::OsRng::new()?;

    let adjectives: Vec<&str> = (0 .. num_adjectives).map(|_| *rng.choose(&ADJECTIVES).unwrap()).collect();
    let noun: &str = *rng.choose(&ANIMALS).unwrap();
    let id = rng.gen_range(id_range.start, id_range.end);
    let slug = format!("{}-{}-{}", adjectives.join("-"), noun, id);

    Ok(slug)
}

#[test]
fn test_generate_slug() {
    let res = generate_slug(2, (0..10000));
    assert_eq!(res.unwrap().split('-').collect::<Vec<&str>>().len(), 4);
}
