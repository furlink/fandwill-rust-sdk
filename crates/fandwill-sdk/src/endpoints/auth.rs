use reqwest::Method;

use fandwill_vo::auth::{SignInResponseVO, SignInVO, SignUpResponseVO, SignUpVO};
use fandwill_vo::meta::RootResponse;

use crate::error::Error;
use crate::frb_dispatch;

frb_dispatch! {
    pub async fn root = root_impl (client) -> Result<RootResponse, Error> {
        let builder = client.request(Method::GET, "")?;
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn sign_up = sign_up_impl (client, body: &SignUpVO) -> Result<SignUpResponseVO, Error> {
        let builder = client.request(Method::POST, "auth/sign-up")?.json(body);
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn sign_in = sign_in_impl (client, body: &SignInVO) -> Result<SignInResponseVO, Error> {
        let builder = client.request(Method::POST, "auth/sign-in")?.json(body);
        client.send_json(builder).await
    }
}
