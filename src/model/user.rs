#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}
