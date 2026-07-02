use reqwest::{Client, Method, RequestBuilder};
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
        let mut base_url = base_url
            .into_url()
            .map_err(|e| Error::Request(e.to_string()))?;
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
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        Ok(serde_json::from_str(&body)?)
    }

    pub(crate) async fn send_empty(&self, builder: RequestBuilder) -> Result<(), Error> {
        let response = builder.send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        Ok(())
    }
}
