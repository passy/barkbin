use diesel;
use std;

#[derive(Fail, Debug)]
pub enum DBError {
    #[fail(display = "Failed to save to database: {}", _0)]
    SaveError(#[cause] diesel::result::Error),

    #[fail(display = "Failed to load from database: {}", _0)]
    LoadError(#[cause] diesel::result::Error),

    #[fail(display = "I/O error: {}", _0)]
    IOError(#[cause] std::io::Error),

    #[fail(display = "Failed to find available unique identifier")]
    UniquenessSaveError
}

impl From<diesel::result::Error> for DBError {
    fn from(error: diesel::result::Error) -> Self {
        DBError::SaveError(error)
    }
}


impl From<std::io::Error> for DBError {
    fn from(error: std::io::Error) -> Self {
        DBError::IOError(error)
    }
}
