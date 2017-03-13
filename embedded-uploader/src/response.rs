#[derive(Deserialize)]
pub struct Response {
    pub status: String,
    pub err: Option<String>,
}
