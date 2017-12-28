extern crate rocket;

use rocket::request::FromParam;
use rocket::http::RawStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Slug {
    pub slug: String,
}

impl<'r> FromParam<'r> for Slug {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let str = param.html_escape().into_owned();

        Ok(Slug { slug: str })
    }
}