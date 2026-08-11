use reqwest::Method;

use fandwill_vo::{
    collections::{AddToCollectionRequest, CollectionEntryVO},
    listings::ListingsVO,
    pagination::{PagedResponse, PaginationParams},
    reviews::ReviewsVO,
    users::{UserCapabilitiesVO, UsersVO},
};

use crate::{client::FandwillClient, error::Error};

impl FandwillClient {
    pub async fn get_me(&self) -> Result<UsersVO, Error> {
        let builder = self.request(Method::GET, "users/me")?;
        self.send_json(builder).await
    }

    pub async fn get_my_capabilities(&self) -> Result<UserCapabilitiesVO, Error> {
        let builder = self.request(Method::GET, "users/me/capabilities")?;
        self.send_json(builder).await
    }

    pub async fn list_collections(
        &self,
        params: &PaginationParams,
    ) -> Result<PagedResponse<CollectionEntryVO>, Error> {
        let builder = self
            .request(Method::GET, "users/me/collections")?
            .query(params);
        self.send_json(builder).await
    }

    pub async fn add_to_collections(
        &self,
        body: &AddToCollectionRequest,
    ) -> Result<CollectionEntryVO, Error> {
        let builder = self
            .request(Method::POST, "users/me/collections")?
            .json(body);
        self.send_json(builder).await
    }

    pub async fn remove_from_collections(&self, entry_id: &str) -> Result<(), Error> {
        let builder = self.request(Method::DELETE, &format!("users/me/collections/{entry_id}"))?;
        self.send_empty(builder).await
    }

    pub async fn get_my_pending_listings(
        &self,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = self.request(Method::GET, "users/me/pending")?.query(params);
        self.send_json(builder).await
    }

    pub async fn get_my_recommendations(
        &self,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = self
            .request(Method::GET, "users/me/recommendations")?
            .query(params);
        self.send_json(builder).await
    }

    pub async fn get_user(&self, id: &str) -> Result<UsersVO, Error> {
        let builder = self.request(Method::GET, &format!("users/{id}"))?;
        self.send_json(builder).await
    }

    pub async fn get_user_bookmarks(
        &self,
        id: &str,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = self
            .request(Method::GET, &format!("users/{id}/bookmarks"))?
            .query(params);
        self.send_json(builder).await
    }

    pub async fn get_user_listings(
        &self,
        id: &str,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = self
            .request(Method::GET, &format!("users/{id}/listings"))?
            .query(params);
        self.send_json(builder).await
    }

    pub async fn get_user_reviews(
        &self,
        id: &str,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ReviewsVO>, Error> {
        let builder = self
            .request(Method::GET, &format!("users/{id}/reviews"))?
            .query(params);
        self.send_json(builder).await
    }
}
