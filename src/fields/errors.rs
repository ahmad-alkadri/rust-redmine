use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorsResult {
    pub errors: Vec<String>,
}
