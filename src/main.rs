#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate failure;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_infer_schema;
extern crate dotenv;

mod db;

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
