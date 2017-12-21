#[derive(Queryable)]
pub struct Bark {
    pub id: i32,
    pub uuid: Vec<u8>,
    pub filename: String,
    pub body: String,
}
