#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(String),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("unexpected status {status}: {body}")]
    Status { status: u16, body: String },

    #[error("failed to parse response JSON: {0}")]
    Json(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Request(e.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::InvalidUrl(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}
