use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::deserialize_trimmed_string;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CheckpointReport {
    pub id: Uuid,
    pub checkpoint_id: Uuid,
    pub title: String,
    pub content: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TeamReport {
    pub id: Uuid,
    pub team_id: Uuid,
    pub checkpoint_id: Option<Uuid>,
    pub title: String,
    pub content: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCheckpointReportPayload {
    pub checkpoint_id: Uuid,

    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Report title cannot be empty"))]
    pub title: String,

    pub content: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTeamReportPayload {
    pub team_id: Uuid,
    pub checkpoint_id: Option<Uuid>,

    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Report title cannot be empty"))]
    pub title: String,

    pub content: Option<serde_json::Value>,
}
