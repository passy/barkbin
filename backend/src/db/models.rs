use chrono;
use db::schema::barks;

#[derive(Queryable)]
pub struct Bark {
    pub id: i32,
    pub slug: String,
    pub filename: String,
    pub body: String,
    pub datetime: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="barks"]
pub struct NewBark<'a> {
    pub filename: &'a str,
    pub body: &'a str,
    pub slug: &'a str,
    pub datetime: &'a chrono::NaiveDateTime,
}
