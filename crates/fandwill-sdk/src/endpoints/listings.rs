use reqwest::Method;

use fandwill_vo::listings::{
    CreateListingVO, ListingVersionVO, ListingsVO, UpdateListingVO, UpdateListingVersionStatusVO,
};

use crate::client::FandwillClient;
use crate::error::Error;
use crate::query::ListingsQuery;
use crate::response::PagedResponse;

impl FandwillClient {
    pub async fn get_listings(
        &self,
        params: &ListingsQuery,
    ) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = self.request(Method::GET, "listings")?.query(params);
        self.send_json(builder).await
    }

    pub async fn get_listing(&self, id: &str) -> Result<ListingsVO, Error> {
        let builder = self.request(Method::GET, &format!("listings/{id}"))?;
        self.send_json(builder).await
    }

    pub async fn add_listing(&self, body: &CreateListingVO) -> Result<ListingsVO, Error> {
        let builder = self.request(Method::POST, "listings")?.json(body);
        self.send_json(builder).await
    }

    pub async fn update_listing(
        &self,
        id: &str,
        body: &UpdateListingVO,
    ) -> Result<ListingsVO, Error> {
        let builder = self
            .request(Method::PUT, &format!("listings/{id}"))?
            .json(body);
        self.send_json(builder).await
    }

    pub async fn delete_listing(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::DELETE, &format!("listings/{id}"))?;
        self.send_empty(builder).await
    }

    pub async fn bookmark_listing(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::POST, &format!("listings/{id}/bookmark"))?;
        self.send_empty(builder).await
    }

    pub async fn unbookmark_listing(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::DELETE, &format!("listings/{id}/bookmark"))?;
        self.send_empty(builder).await
    }

    pub async fn get_listing_versions(&self, id: &str) -> Result<Vec<ListingVersionVO>, Error> {
        let builder = self.request(Method::GET, &format!("listings/{id}/versions"))?;
        self.send_json(builder).await
    }

    pub async fn update_listing_version_status(
        &self,
        ref_id: &str,
        version_id: &str,
        body: &UpdateListingVersionStatusVO,
    ) -> Result<ListingVersionVO, Error> {
        let builder = self
            .request(
                Method::PATCH,
                &format!("listings/{ref_id}/versions/{version_id}/status"),
            )?
            .json(body);
        self.send_json(builder).await
    }
}
