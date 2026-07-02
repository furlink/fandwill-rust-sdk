use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct UsersVO {
    pub id: String,
    pub sub: String,
}
