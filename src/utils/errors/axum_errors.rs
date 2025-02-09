use serde::Serialize;

#[derive(Serialize)]
pub struct PathError {
    pub message: String,
    pub location: Option<String>,
}
