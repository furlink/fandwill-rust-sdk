use reqwest::Method;

use fandwill_vo::listings::{CreateListingVO, ListingVersionVO, ListingsVO, UpdateListingVO};

use crate::error::Error;
use crate::frb_dispatch;
use crate::query::ListingsQuery;
use crate::response::PagedResponse;

frb_dispatch! {
    pub async fn get_listings = get_listings_impl (client, params: &ListingsQuery) -> Result<PagedResponse<ListingsVO>, Error> {
        let builder = client.request(Method::GET, "listings")?.query(params);
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn get_listing = get_listing_impl (client, id: &str) -> Result<ListingsVO, Error> {
        let builder = client.request(Method::GET, &format!("listings/{id}"))?;
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn add_listing = add_listing_impl (client, body: &CreateListingVO) -> Result<ListingsVO, Error> {
        let builder = client.request(Method::POST, "listings")?.json(body);
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn update_listing = update_listing_impl (client, id: &str, body: &UpdateListingVO) -> Result<ListingsVO, Error> {
        let builder = client.request(Method::PUT, &format!("listings/{id}"))?.json(body);
        client.send_json(builder).await
    }
}

frb_dispatch! {
    pub async fn delete_listing = delete_listing_impl (client, id: &str) -> Result<(), Error> {
        let builder = client.request(Method::DELETE, &format!("listings/{id}"))?;
        client.send_empty(builder).await
    }
}

frb_dispatch! {
    pub async fn bookmark_listing = bookmark_listing_impl (client, id: &str) -> Result<(), Error> {
        let builder = client.request(Method::POST, &format!("listings/{id}/bookmark"))?;
        client.send_empty(builder).await
    }
}

frb_dispatch! {
    pub async fn unbookmark_listing = unbookmark_listing_impl (client, id: &str) -> Result<(), Error> {
        let builder = client.request(Method::DELETE, &format!("listings/{id}/bookmark"))?;
        client.send_empty(builder).await
    }
}

frb_dispatch! {
    pub async fn get_listing_versions = get_listing_versions_impl (client, id: &str) -> Result<Vec<ListingVersionVO>, Error> {
        let builder = client.request(Method::GET, &format!("listings/{id}/versions"))?;
        client.send_json(builder).await
    }
}
