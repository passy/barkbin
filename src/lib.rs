#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate failure;
#[macro_use] extern crate diesel;
extern crate dotenv;

pub mod db;
