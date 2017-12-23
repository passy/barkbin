#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate barkbin_backend;

use barkbin_backend::db::{create_bark, establish_connection};
use rocket::Data;
use rocket::response::status;
use std::io::Read;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn create(paste: Data) -> Result<status::Created<String>, status::BadRequest<()>> {
    let filename = "test.txt";
    let mut body = String::new();
    paste.open().read_to_string(&mut body).unwrap();

    let conn = establish_connection().expect("Don't do this you idiot!");

    match create_bark(&conn, &filename, &body) {
        Ok(_) => Ok(status::Created("Cool.".to_owned(), None)),
        Err(_) => Err(status::BadRequest(None)),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, create]).launch();
}