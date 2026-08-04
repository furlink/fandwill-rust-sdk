use reqwest::{Client, Method, RequestBuilder, header::LOCATION, redirect::Policy};
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::Auth;
use crate::error::Error;

#[derive(Clone)]
pub struct FandwillClient {
    pub(crate) http: Client,
    pub(crate) base_url: Url,
    pub(crate) auth: Option<Auth>,
}

impl FandwillClient {
    pub fn new(base_url: impl reqwest::IntoUrl) -> Result<Self, Error> {
        let mut base_url = base_url.into_url().map_err(Error::Request)?;
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }
        let http = Client::builder().build()?;

        Ok(Self {
            http,
            base_url,
            auth: None,
        })
    }

    /// Configures the legacy `X-Api-Key` authentication header.
    ///
    /// The current public OpenAPI contract only declares bearer JWT authentication.
    #[deprecated(
        note = "the current Fandwill OpenAPI contract only supports bearer JWTs; use with_jwt"
    )]
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.auth = Some(Auth::api_key(key));
        self
    }

    pub fn with_jwt(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(Auth::jwt(token));
        self
    }

    pub(crate) fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let url = self.base_url.join(path)?;
        let mut builder = self.http.request(method, url);

        if let Some(auth) = &self.auth {
            builder = match auth {
                Auth::ApiKey(key) => builder.header("X-Api-Key", key),
                Auth::Jwt(token) => builder.bearer_auth(token),
            };
        }

        Ok(builder)
    }

    pub(crate) async fn send_json<T: DeserializeOwned>(
        &self,
        builder: RequestBuilder,
    ) -> Result<T, Error> {
        let response = builder.send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(Error::Status { status, body });
        }
        Ok(serde_json::from_str(&body)?)
    }

    pub(crate) async fn send_empty(&self, builder: RequestBuilder) -> Result<(), Error> {
        let response = builder.send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(Error::Status { status, body });
        }
        Ok(())
    }

    pub(crate) async fn send_redirect_url(&self, builder: RequestBuilder) -> Result<Url, Error> {
        // A request can be executed by another reqwest client after it is built. Use a
        // no-redirect client so the SDK returns the presigned target without downloading it.
        let request = builder.build()?;
        let client = Client::builder().redirect(Policy::none()).build()?;
        let response = client.execute(request).await?;
        let status = response.status();

        if !status.is_redirection() {
            let body = response.text().await?;
            return Err(Error::Status { status, body });
        }

        let base_url = response.url().clone();
        let location = response
            .headers()
            .get(LOCATION)
            .ok_or(Error::MissingRedirectLocation { status })?
            .to_str()
            .map_err(|error| Error::InvalidRedirectLocation(error.to_string()))?;

        base_url
            .join(location)
            .map_err(|error| Error::InvalidRedirectLocation(format!("{location:?}: {error}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_reqwest_build_exposes_rustls() {
        Client::builder().use_rustls_tls().build().unwrap();
    }
}
