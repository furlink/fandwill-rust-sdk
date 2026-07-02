/// How the client authenticates with the API.
#[derive(Clone, Debug)]
pub enum Auth {
    /// API key (sent as `X-Api-Key`).
    ApiKey(String),
    /// JWT access token (sent as `Authorization: Bearer`).
    Jwt(String),
}

impl Auth {
    pub fn api_key(key: impl Into<String>) -> Self {
        Self::ApiKey(key.into())
    }

    pub fn jwt(token: impl Into<String>) -> Self {
        Self::Jwt(token.into())
    }
}
