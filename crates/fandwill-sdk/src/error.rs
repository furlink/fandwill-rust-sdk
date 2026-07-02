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

#[cfg(feature = "frb")]
#[derive(Debug)]
pub enum FrbError {
    Request(String),
    InvalidUrl(String),
    Status { status: u16, body: String },
    Json(String),
}

#[cfg(feature = "frb")]
macro_rules! frb_error_from {
    ($src:ident => $dst:ident, Status $(, $rest:ident)* $(,)?) => {
        impl From<$src> for $dst {
            fn from(e: $src) -> Self {
                match e {
                    $src::Status { status, body } => Self::Status {
                        status: status.as_u16(),
                        body,
                    },
                    $( $src::$rest(v) => Self::$rest(v.to_string()), )*
                }
            }
        }
    };
}

#[cfg(feature = "frb")]
frb_error_from!(Error => FrbError, Status, Request, InvalidUrl, Json);
