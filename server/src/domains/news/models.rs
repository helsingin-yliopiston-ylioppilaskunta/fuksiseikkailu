use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::{deserialize_trimmed_option_string, deserialize_trimmed_string};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct NewsArticle {
    pub id: Uuid,
    pub title: String,
    pub content: serde_json::Value,
    pub published_at: Option<DateTime<Utc>>,
    pub notification_sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNewsPayload {
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    pub content: serde_json::Value,

    /// Set to publish immediately or schedule for future publication
    pub published_at: Option<DateTime<Utc>>,

    /// Optional flag to trigger push notification upon creation
    pub send_push: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateNewsPayload {
    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    pub title: Option<String>,

    pub content: Option<serde_json::Value>,

    pub published_at: Option<DateTime<Utc>>,
}
