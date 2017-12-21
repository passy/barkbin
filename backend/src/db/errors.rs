use diesel;

#[derive(Fail, Debug)]
pub enum DBError {
    #[fail(display = "Failed to save to database: {}", _0)]
    SaveError(#[cause] diesel::result::Error),

    #[fail(display = "Failed to find available unique identifier")]
    UniquenessSaveError
}

impl From<diesel::result::Error> for DBError {
    fn from(error: diesel::result::Error) -> Self {
        DBError::SaveError(error)
    }
}
