use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::trim::{deserialize_trimmed_option_string, deserialize_trimmed_string};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Area {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateArea {
    #[serde(deserialize_with = "deserialize_trimmed_string")]
    #[validate(length(min = 1, message = "Area name cannot be empty"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateArea {
    #[serde(default, deserialize_with = "deserialize_trimmed_option_string")]
    #[validate(length(min = 1, message = "Area name cannot be empty"))]
    pub name: Option<String>,
}
