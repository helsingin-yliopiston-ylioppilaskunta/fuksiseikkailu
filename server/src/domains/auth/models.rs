use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{domains::users::models::Role, utils::trim::deserialize_trimmed_option_string};

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthTokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterPayload {
    pub role: Role,

    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    #[validate(email(message = "Invalid email address format"))]
    pub email: Option<String>,

    pub checkpoint_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RequestOtpPayload {
    #[serde(deserialize_with = "deserialize_trimmed_option_string")]
    #[validate(email(message = "Invalid email address format"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyOtpPayload {
    pub email: String,
    #[validate(length(min = 6, max = 6, message = "OTP code must be exactly 6 digits"))]
    pub code: String,
}
