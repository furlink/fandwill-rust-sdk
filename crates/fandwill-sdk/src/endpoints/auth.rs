use reqwest::Method;

use fandwill_vo::auth::{SignInResponseVO, SignInVO, SignUpResponseVO, SignUpVO};
use fandwill_vo::meta::RootResponse;

use crate::client::FandwillClient;
use crate::error::Error;

impl FandwillClient {
    pub async fn root(&self) -> Result<RootResponse, Error> {
        let builder = self.request(Method::GET, "")?;
        self.send_json(builder).await
    }

    pub async fn sign_up(&self, body: &SignUpVO) -> Result<SignUpResponseVO, Error> {
        let builder = self.request(Method::POST, "auth/sign-up")?.json(body);
        self.send_json(builder).await
    }

    pub async fn sign_in(&self, body: &SignInVO) -> Result<SignInResponseVO, Error> {
        let builder = self.request(Method::POST, "auth/sign-in")?.json(body);
        self.send_json(builder).await
    }
}
