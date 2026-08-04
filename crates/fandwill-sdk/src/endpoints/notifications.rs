use reqwest::Method;

use fandwill_vo::{
    notifications::{
        MarkAllNotificationsReadVO, NotificationSummaryVO, NotificationVO, NotificationsQuery,
    },
    pagination::PagedResponse,
};

use crate::{client::FandwillClient, error::Error};

impl FandwillClient {
    pub async fn list_notifications(
        &self,
        query: &NotificationsQuery,
    ) -> Result<PagedResponse<NotificationVO>, Error> {
        let builder = self
            .request(Method::GET, "users/me/notifications")?
            .query(query);
        self.send_json(builder).await
    }

    pub async fn mark_all_notifications_read(&self) -> Result<MarkAllNotificationsReadVO, Error> {
        let builder = self.request(Method::PUT, "users/me/notifications")?;
        self.send_json(builder).await
    }

    pub async fn get_notification_summary(&self) -> Result<NotificationSummaryVO, Error> {
        let builder = self.request(Method::GET, "users/me/notifications/summary")?;
        self.send_json(builder).await
    }

    pub async fn mark_notification_read(
        &self,
        notification_id: &str,
    ) -> Result<NotificationVO, Error> {
        let builder = self.request(
            Method::PUT,
            &format!("users/me/notifications/{notification_id}/read"),
        )?;
        self.send_json(builder).await
    }
}
