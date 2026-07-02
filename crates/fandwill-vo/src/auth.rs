use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(deny_unknown_fields)]
pub struct SignUpVO {
    #[garde(email)]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[garde(length(min = 8))]
    #[schema(example = "password123")]
    pub password: String,

    #[garde(skip)]
    #[schema(example = "TestUser")]
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SignUpResponseVO {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(untagged, deny_unknown_fields)]
pub enum SignInVO {
    Email {
        #[garde(email)]
        #[schema(example = "user@example.com")]
        email: String,

        #[garde(length(min = 8))]
        #[schema(example = "password123")]
        password: String,
    },
    Phone {
        #[garde(phone_number)]
        #[schema(example = "+861380000000")]
        phone: String,

        #[garde(length(min = 8))]
        #[schema(example = "password123")]
        password: String,
    },
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct SignInResponseVO {
    pub token: String,
}
