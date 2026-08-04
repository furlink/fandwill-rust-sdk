use reqwest::StatusCode;
use serde::Deserialize;

/// Structured error body returned by the Fandwill API.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ApiError {
    pub code: u16,
    pub msg: String,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("unexpected status {status}: {body}")]
    Status { status: StatusCode, body: String },

    #[error("failed to parse response JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("redirect response {status} did not include a Location header")]
    MissingRedirectLocation { status: StatusCode },

    #[error("invalid redirect Location header: {0}")]
    InvalidRedirectLocation(String),
}

impl Error {
    /// Returns the HTTP status for a non-success API response.
    pub const fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Status { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Returns the unmodified response body for a non-success API response.
    pub fn body(&self) -> Option<&str> {
        match self {
            Self::Status { body, .. } => Some(body),
            _ => None,
        }
    }

    /// Parses the documented `{ "code", "msg" }` body without discarding the raw body.
    pub fn api_error(&self) -> Option<ApiError> {
        let Self::Status { body, .. } = self else {
            return None;
        };
        serde_json::from_str(body).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_api_error_is_available_without_losing_raw_body() {
        let body = r#"{"code":401,"msg":"unauthorized"}"#.to_owned();
        let error = Error::Status {
            status: StatusCode::FORBIDDEN,
            body: body.clone(),
        };

        assert_eq!(error.status(), Some(StatusCode::FORBIDDEN));
        assert_eq!(error.body(), Some(body.as_str()));
        assert_eq!(
            error.api_error(),
            Some(ApiError {
                code: 401,
                msg: "unauthorized".into(),
            })
        );
    }

    #[test]
    fn malformed_api_error_keeps_status_and_raw_body() {
        for body in ["", "<html>", "{", r#"{"code":400}"#] {
            let error = Error::Status {
                status: StatusCode::BAD_REQUEST,
                body: body.to_owned(),
            };
            assert_eq!(error.status(), Some(StatusCode::BAD_REQUEST));
            assert_eq!(error.body(), Some(body));
            assert_eq!(error.api_error(), None);
        }
    }
}
