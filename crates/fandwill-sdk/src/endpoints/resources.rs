use reqwest::Method;
use url::Url;

use fandwill_vo::resources::{CreateResourceVO, ResourceVO};

use crate::{client::FandwillClient, error::Error};

impl FandwillClient {
    pub async fn create_resource(&self) -> Result<CreateResourceVO, Error> {
        let builder = self.request(Method::POST, "resources")?;
        self.send_json(builder).await
    }

    /// Returns the presigned download target from the API's redirect without downloading it.
    pub async fn get_resource(&self, id: &str) -> Result<Url, Error> {
        let builder = self.request(Method::GET, &format!("resources/{id}"))?;
        self.send_redirect_url(builder).await
    }

    pub async fn get_resource_metadata(&self, id: &str) -> Result<ResourceVO, Error> {
        let builder = self.request(Method::GET, &format!("resources/{id}/metadata"))?;
        self.send_json(builder).await
    }
}
