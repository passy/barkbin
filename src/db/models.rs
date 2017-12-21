#[derive(Queryable)]
pub struct Bark {
    id: i32,
    uuid: String,
    filename: String,
    body: String,
}
