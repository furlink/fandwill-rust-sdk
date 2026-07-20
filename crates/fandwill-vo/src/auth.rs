use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
#[serde(deny_unknown_fields)]
pub struct SignUpVO {
    #[cfg_attr(feature = "garde", garde(email))]
    #[cfg_attr(feature = "utoipa", schema(example = "user@example.com"))]
    pub email: String,

    #[cfg_attr(feature = "garde", garde(length(min = 8)))]
    #[cfg_attr(feature = "utoipa", schema(example = "password123"))]
    pub password: String,

    #[cfg_attr(feature = "garde", garde(skip))]
    #[cfg_attr(feature = "utoipa", schema(example = "TestUser"))]
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(deny_unknown_fields)]
pub struct SignUpResponseVO {
    pub iam_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "garde", derive(garde::Validate))]
#[serde(untagged, deny_unknown_fields)]
pub enum SignInVO {
    Email {
        #[cfg_attr(feature = "garde", garde(email))]
        #[cfg_attr(feature = "utoipa", schema(example = "user@example.com"))]
        email: String,

        #[cfg_attr(feature = "garde", garde(length(min = 8)))]
        #[cfg_attr(feature = "utoipa", schema(example = "password123"))]
        password: String,
    },
    Phone {
        #[cfg_attr(feature = "garde", garde(phone_number))]
        #[cfg_attr(feature = "utoipa", schema(example = "+861380000000"))]
        phone: String,

        #[cfg_attr(feature = "garde", garde(length(min = 8)))]
        #[cfg_attr(feature = "utoipa", schema(example = "password123"))]
        password: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(deny_unknown_fields)]
pub struct SignInResponseVO {
    pub token: String,
}
