use reqwest::Method;

use fandwill_vo::{
    pagination::{PagedResponse, PaginationParams},
    reviews::{CreateReplyVO, CreateReviewVO, ReviewFilter, ReviewReplyVO, ReviewsVO},
    validation::{ReviewReplyVOWithValidation, ReviewsVOWithValidation},
};

use crate::{client::FandwillClient, error::Error};

impl FandwillClient {
    pub async fn get_reviews(
        &self,
        filter: &ReviewFilter,
    ) -> Result<PagedResponse<ReviewsVO>, Error> {
        let builder = self.request(Method::GET, "reviews")?.query(filter);
        self.send_json(builder).await
    }

    pub async fn add_review(
        &self,
        body: &CreateReviewVO,
    ) -> Result<ReviewsVOWithValidation, Error> {
        let builder = self.request(Method::POST, "reviews")?.json(body);
        self.send_json(builder).await
    }

    pub async fn get_review(&self, id: &str) -> Result<ReviewsVO, Error> {
        let builder = self.request(Method::GET, &format!("reviews/{id}"))?;
        self.send_json(builder).await
    }

    pub async fn delete_review(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::DELETE, &format!("reviews/{id}"))?;
        self.send_empty(builder).await
    }

    pub async fn like_review(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::POST, &format!("reviews/{id}/like"))?;
        self.send_empty(builder).await
    }

    pub async fn unlike_review(&self, id: &str) -> Result<(), Error> {
        let builder = self.request(Method::DELETE, &format!("reviews/{id}/like"))?;
        self.send_empty(builder).await
    }

    pub async fn get_replies(
        &self,
        id: &str,
        params: &PaginationParams,
    ) -> Result<PagedResponse<ReviewReplyVO>, Error> {
        let builder = self
            .request(Method::GET, &format!("reviews/{id}/replies"))?
            .query(params);
        self.send_json(builder).await
    }

    pub async fn add_reply(
        &self,
        id: &str,
        body: &CreateReplyVO,
    ) -> Result<ReviewReplyVOWithValidation, Error> {
        let builder = self
            .request(Method::POST, &format!("reviews/{id}/replies"))?
            .json(body);
        self.send_json(builder).await
    }
}
