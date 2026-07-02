use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("unexpected status {status}: {body}")]
    Status { status: StatusCode, body: String },

    #[error("failed to parse response JSON: {0}")]
    Json(#[from] serde_json::Error),
}
