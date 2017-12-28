#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate barkbin_backend;
extern crate barkbin_common as common;

use barkbin_backend::db::{create_bark, establish_connection, load_bark};
use self::common::Slug;
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

      GET /b/<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/b/<id>")]
fn get_bark(id: Slug) -> Result<String, status::Custom<String>> {
    let conn = establish_connection()
        .map_err(|_| status::Custom(rocket::http::Status::InternalServerError, "Can't establish DB connection".into()))?;

    let bark = load_bark(&conn, &id)
        .map_err(|e| status::Custom(rocket::http::Status::NotFound, format!("Cannot load bark: {:?}", e)))?;

    Ok(bark.body)
}

#[post("/", data = "<paste>")]
fn create(paste: Data) -> Result<status::Created<String>, status::BadRequest<()>> {
    let filename = "test.txt";
    let mut body = String::new();
    paste.open().read_to_string(&mut body).unwrap();

    let conn = establish_connection().map_err(|_| status::BadRequest(None))?;

    match create_bark(&conn, &filename, &body) {
        Err(_) => Err(status::BadRequest(None)),
        Ok(res) => {
            let url = format!("/b/{}", res.slug);
            Ok(status::Created(url, Some(res.slug)))
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, create, get_bark]).launch();
}