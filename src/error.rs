/// Client error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
}
