use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::deserialize_trimmed_string;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Photo {
    pub id: Uuid,
    pub s3_key: String,
    pub url: String,
    pub team_id: Option<Uuid>,
    pub checkpoint_id: Option<Uuid>,
    pub published: bool,
    pub vote_count: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePhotoPayload {
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "s3_key cannot be empty"))]
    pub s3_key: String,

    pub team_id: Option<Uuid>,
    pub checkpoint_id: Option<Uuid>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePhotoPayload {
    pub team_id: Option<Uuid>,
    pub checkpoint_id: Option<Uuid>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitSuggestionPayload {
    pub suggested_team_number: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PhotoTeamSuggestion {
    pub id: Uuid,
    pub photo_id: Uuid,
    pub suggested_team_number: i32,
    pub voter_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}
